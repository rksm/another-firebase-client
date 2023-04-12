use anyhow::Result;
use serde_json::{json, Map, Value};

/// `ObservedValue` holds the state received by a realtime db listener and
/// updates it when changes come in.
#[derive(Debug, PartialEq, Eq)]
pub struct ObservedValue(Value);

impl ObservedValue {
    pub fn new() -> Self {
        Self::value(Value::Null)
    }

    pub fn value(value: Value) -> Self {
        Self(value)
    }

    pub fn apply_put(self, action: PutAction) -> Result<(Vec<String>, Self)> {
        let PutAction { path, data } = action;
        let value = apply_put(&path, self.0, data)?;
        Ok((path, Self(value)))
    }

    pub fn apply_patch(self, action: PatchAction) -> Result<(Vec<String>, Self)> {
        let PatchAction { path, data } = action;

        let patch_value = match data {
            Value::Object(obj) => obj,
            _ => return Err(anyhow::anyhow!("put value is not an object")),
        };

        // keys of the object we receive can contain '/' like {"a/b/c": {...}}.
        // In that case, create a nested object structure like {"a": {"b": {"c": {...}}}}
        let patch_value = patch_value
            .into_iter()
            .map(|(key, val)| {
                if !key.contains('/') {
                    return Value::Object(Map::from_iter([(key, val)]));
                }
                key.split('/').rfold(val, |val, key| {
                    Value::Object(Map::from_iter([(key.to_string(), val)]))
                })
            })
            .fold(Map::new(), |mut outer, val| match val {
                Value::Object(obj) => {
                    for (key, val) in obj.into_iter() {
                        outer.insert(key, val);
                    }
                    outer
                }
                _ => outer,
            });

        let value = apply_patch(&path, self.0, Value::Object(patch_value))?;
        Ok((path, Self(value)))
    }
}

impl AsRef<Value> for ObservedValue {
    fn as_ref(&self) -> &Value {
        &self.0
    }
}

#[derive(Debug)]
pub enum Action {
    Put(PutAction),
    Patch(PatchAction),
}

impl Action {
    pub fn path(&self) -> &[String] {
        match self {
            Action::Put(PutAction { path, .. }) => path,
            Action::Patch(PatchAction { path, .. }) => path,
        }
    }
}

#[derive(Debug)]
pub struct PutAction {
    pub path: Vec<String>,
    pub data: Value,
}

impl<S> From<(S, Value)> for PutAction
where
    S: ToString,
{
    fn from((path, data): (S, Value)) -> Self {
        let path = parse_path(path.to_string());
        Self { path, data }
    }
}

#[derive(Debug)]
pub struct PatchAction {
    pub path: Vec<String>,
    pub data: Value,
}

impl<S> From<(S, Value)> for PatchAction
where
    S: ToString,
{
    fn from((path, data): (S, Value)) -> Self {
        let path = parse_path(path.to_string());
        Self { path, data }
    }
}

fn apply_put(path: &[String], data: Value, put_value: Value) -> Result<Value> {
    modify_path(path, data, move |_| put_value)
}

fn convert_array_to_object(value: Value) -> Value {
    let mut obj = serde_json::Map::new();
    if let Value::Array(arr) = value {
        for (i, val) in arr.into_iter().enumerate() {
            obj.insert(i.to_string(), val);
        }
    }
    Value::Object(obj)
}

/// Merges two JSON values together
fn merge_values(a: Value, b: Value) -> Value {
    let a = if a.is_array() {
        convert_array_to_object(a)
    } else {
        a
    };

    match (a, b) {
        (Value::Object(mut a), Value::Object(b)) => {
            for (key, val_b) in b.into_iter() {
                match a.remove(&key) {
                    Some(val_a) => a.insert(key, merge_values(val_a, val_b)),
                    None => a.insert(key, val_b),
                };
            }
            Value::Object(a)
        }
        (_, b) => b,
    }
}

fn apply_patch(path: &[String], data: Value, patch_value: Value) -> Result<Value> {
    tracing::trace!(
        "[apply_patch] patch={:?} data={:?} patch={:?}",
        path,
        data,
        patch_value
    );

    modify_path(path, data, |value| merge_values(value, patch_value))
}

#[derive(Debug)]
enum Key<'a> {
    String(&'a str),
    Number(usize),
}

fn modify_path(
    path: &[String],
    data: Value,
    apply_fn: impl FnOnce(Value) -> Value,
) -> Result<Value> {
    let mut stack = Vec::new();

    let mut current = data;

    for p in path {
        let (key, inner) = if current.is_array() {
            match p.parse() {
                Err(_) => {
                    // current is an array put the key we got is not a number...
                    // convert the array into an object!
                    current = convert_array_to_object(current);
                    (Key::String(p), Value::Null)
                }
                Ok(index) => {
                    // We have an array and an index into it. Make sure it is
                    // large enough.
                    let arr = current.as_array_mut().unwrap();
                    if arr.len() <= index {
                        for _ in arr.len()..index + 1 {
                            arr.push(Value::Null);
                        }
                    }

                    (
                        Key::Number(index),
                        std::mem::replace(&mut arr[index], Value::Null),
                    )
                }
            }
        } else if current.is_object() {
            let obj = current.as_object_mut().unwrap();
            (Key::String(p), obj.remove(p).unwrap_or(Value::Null))
        } else {
            current = json![{}];
            (Key::String(p), Value::Null)
        };

        tracing::trace!("[modify_path] -> {:?} {:?}", key, current);

        stack.push((key, current));
        current = inner;
    }

    tracing::trace!("[modify_path] <-> APPLY {:?}", current);
    current = apply_fn(current);
    tracing::trace!("[modify_path] <-> RESULT {:?}", current);

    for (p, mut value) in stack.into_iter().rev() {
        tracing::trace!("[modify_path] <- {:?} {:?}", p, current);

        if value.is_array() {
            let arr = value.as_array_mut().unwrap();
            let index = match p {
                Key::Number(index) => index,
                _ => return Err(anyhow::anyhow!("Expected key to be index into array")),
            };
            arr[index] = current;
        } else {
            let obj = value.as_object_mut().unwrap();
            let key = match p {
                Key::String(key) => key,
                _ => return Err(anyhow::anyhow!("Expected key to be string")),
            };

            obj.insert(key.to_string(), current);
        };
        current = value;
    }

    Ok(current)
}

fn parse_path(path: String) -> Vec<String> {
    let parts = path
        .split('/')
        .filter_map(|ea| {
            if ea.is_empty() {
                None
            } else {
                Some(ea.to_string())
            }
        })
        .collect::<Vec<_>>();
    parts
}

#[cfg(test)]
mod tests {
    use super::{parse_path, ObservedValue};
    use crate::rdb::listener_updates::PatchAction;

    use anyhow::Result;
    use pretty_assertions::assert_eq;
    use serde_json::{json, Value};

    // -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
    // parse path

    #[test]
    fn parse_path_root() -> Result<()> {
        let result = parse_path("/".to_string());
        assert_eq!(result, Vec::<&str>::new());
        Ok(())
    }

    #[test]
    fn parse_path_one_component() -> Result<()> {
        let result = parse_path("/foo".to_string());
        assert_eq!(result, vec!["foo"]);
        Ok(())
    }

    #[test]
    fn parse_path_with_number() -> Result<()> {
        let result = parse_path("/foo/3/bar".to_string());
        assert_eq!(result, vec!["foo", "3", "bar"]);
        Ok(())
    }

    // -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
    // put

    #[test]
    fn put_value() -> Result<()> {
        let action = ("", Value::Number(1.into())).into();
        let (_path, val) = ObservedValue::new().apply_put(action)?;
        assert_eq!(val.0, json![1]);
        Ok(())
    }

    #[test]
    fn put_value_path() -> Result<()> {
        let action = ("/foo", Value::Number(1.into())).into();
        let (_path, val) = ObservedValue::new().apply_put(action)?;
        assert_eq!(val.0, json![{"foo": 1}]);
        Ok(())
    }

    #[test]
    fn put_path() -> Result<()> {
        let action = ("/a/b", Value::Null).into();
        let (_path, val) = ObservedValue::new().apply_put(action)?;
        assert_eq!(val.0, json![{"a": {"b": null}}]);
        Ok(())
    }

    #[test]
    fn put_null() -> Result<()> {
        let action = ("/", Value::Null).into();
        let (_path, val) = ObservedValue::value(json![{"a": 1}]).apply_put(action)?;
        assert_eq!(val.0, Value::Null);
        Ok(())
    }

    #[test]
    fn put_overwrite() -> Result<()> {
        let action = ("/foo/bar/baz", json![{"hello": "world"}]).into();
        let (_path, val) = ObservedValue::value(json![{"foo": 1}]).apply_put(action)?;
        assert_eq!(val.0, json![{ "foo": {"bar": {"baz": {"hello": "world"}}}}]);
        Ok(())
    }

    #[test]
    fn put_array() -> Result<()> {
        let action = ("/foo", json![[1, 2, 3]]).into();
        let (_path, val) = ObservedValue::value(json![{"foo": 1}]).apply_put(action)?;
        assert_eq!(val.0, json![{ "foo": [1,2,3]}]);
        Ok(())
    }

    #[test]
    fn put_array_value() -> Result<()> {
        let action = ("foo/1", json!["hello"]).into();
        let (_path, val) = ObservedValue::value(json![{"foo": [1, 2, 3]}]).apply_put(action)?;
        assert_eq!(val.0, json![{ "foo": [1,"hello",3]}]);
        Ok(())
    }

    #[test]
    fn put_array_value_outside() -> Result<()> {
        let action = ("foo/3", json![23]).into();
        let (_path, val) = ObservedValue::value(json![{"foo": [1]}]).apply_put(action)?;
        assert_eq!(val.0, json![{"foo": [1, null, null, 23]}]);
        Ok(())
    }

    #[test]
    fn put_array_value_outside_2() -> Result<()> {
        let action = ("foo/1", json![23]).into();
        let (_path, val) = ObservedValue::value(json![{"foo": [1]}]).apply_put(action)?;
        assert_eq!(val.0, json![{"foo": [1, 23]}]);
        Ok(())
    }

    #[test]
    fn put_into_array() -> Result<()> {
        let action = ("/test/foo", json![[1, 2]]).into();
        let (_path, val) = ObservedValue::value(json![{"test": [1,2]}]).apply_put(action)?;
        assert_eq!(val.0, json![{"test": {"0": 1, "1": 2, "foo": [1,2]}}]);
        Ok(())
    }

    // -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
    // "deep" keys

    #[test]
    fn patch_action_nested_path() -> Result<()> {
        let val = json![{"a":{"b":{"foo": 23}}}];
        let path = "/";
        let data = json![{"a/b":{"foo": 23}}];
        let action: PatchAction = (path, data).into();
        let (_path, val) = ObservedValue::value(val).apply_patch(action)?;
        assert_eq!(val.0, json![{"a":{"b":{"foo": 23}}}]);
        Ok(())
    }

    #[test]
    fn patch_action_nested_path_2() -> Result<()> {
        let path = "/";
        let start_val = json![{"a": {"b": {"c": 3}, "e": 5}, "d": 4}];
        let patch_val = json![{"a/b/e": {"x": 23}}];
        let expected = json![{"a" : {"b" : {"c" : 3, "e" : {"x" : 23}}, "e" : 5}, "d" : 4}];

        let action: PatchAction = (path, patch_val).into();
        let (_path, val) = ObservedValue::value(start_val).apply_patch(action)?;
        assert_eq!(val.0, expected);
        Ok(())
    }

    // -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
    // patch

    #[test]
    fn patch_null() -> Result<()> {
        let action = ("/", json![{"foo":1}]).into();
        let (_path, val) = ObservedValue::new().apply_patch(action)?;
        assert_eq!(val.0, json![{"foo": 1}]);
        Ok(())
    }

    #[test]
    fn patch_object() -> Result<()> {
        let action = ("/", json![{"foo":1, "zork": 9}]).into();
        let (_path, val) = ObservedValue::value(json![{"foo": 3, "bar": 4}]).apply_patch(action)?;
        assert_eq!(val.0, json![{"foo": 1, "bar": 4, "zork": 9}]);
        Ok(())
    }

    #[test]
    fn patch_number() -> Result<()> {
        let action = ("/bar", json![{"foo":1}]).into();
        let (_path, val) = ObservedValue::value(json![{"bar": 1}]).apply_patch(action)?;
        assert_eq!(val.0, json![{"bar": {"foo": 1}}]);
        Ok(())
    }

    #[test]
    fn patch_array() -> Result<()> {
        let action = ("/", json![{"foo":1}]).into();
        let (_path, val) = ObservedValue::value(json![[1]]).apply_patch(action)?;
        assert_eq!(val.0, json![{"0": 1, "foo": 1}]);
        Ok(())
    }

    #[test]
    #[should_panic]
    fn patch_with_non_object() {
        ObservedValue::value(json![[1]])
            .apply_patch(("/", json![1]).into())
            .unwrap();
    }
}
