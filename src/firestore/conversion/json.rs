use firestore_grpc::v1 as firestore;
use prost_types::Timestamp;
use serde::de::DeserializeOwned;
use serde_json::{Map, Number, Value};
use std::collections::HashMap;

use crate::FirestoreConversionError;

use super::{
    FromFirestoreDocument, FromFirestoreValue, IntoFirestoreDocument, IntoFirestoreDocumentValue,
};

// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-

impl FromFirestoreValue for Value {
    type Err = FirestoreConversionError;

    fn convert(val: firestore::Value) -> Result<Value, FirestoreConversionError> {
        let result = match val.value_type {
            None => Value::Null,
            Some(firestore::value::ValueType::NullValue(_)) => Value::Null,
            Some(firestore::value::ValueType::BooleanValue(val)) => Value::Bool(val),
            Some(firestore::value::ValueType::IntegerValue(val)) => Value::Number(val.into()),
            Some(firestore::value::ValueType::DoubleValue(val)) => {
                let n = Number::from_f64(val).unwrap_or_else(|| 0.into());
                Value::Number(n)
            }
            Some(firestore::value::ValueType::TimestampValue(val)) => {
                let ms = std::time::Duration::new(val.seconds as u64, val.nanos as u32).as_millis()
                    as u64;
                Value::Number(ms.into())
            }
            Some(firestore::value::ValueType::StringValue(val)) => Value::String(val),
            Some(firestore::value::ValueType::ArrayValue(val)) => Value::Array(
                val.values
                    .into_iter()
                    .map(|val| Value::convert(val).unwrap())
                    .collect::<Vec<_>>(),
            ),
            Some(firestore::value::ValueType::MapValue(val)) => {
                let mut obj = Map::new();
                for (key, val) in val.fields {
                    obj.insert(key.to_string(), Value::convert(val)?);
                }
                Value::Object(obj)
            }
            Some(firestore::value::ValueType::BytesValue(val)) => {
                eprintln!("Trying to convert firestore BytesValue to JSON... no guarantees");
                let string = String::from_utf8(val).unwrap_or_else(|_| String::new());
                Value::String(string)
            }
            Some(firestore::value::ValueType::ReferenceValue(val)) => {
                eprintln!("Trying to convert firestore ReferenceValue to JSON... no guarantees");
                Value::String(val)
            }
            Some(firestore::value::ValueType::GeoPointValue(val)) => {
                eprintln!("Trying to convert firestore GeoPointValue to JSON... no guarantees");
                let mut obj = Map::new();
                obj.insert(
                    "latitude".to_string(),
                    Number::from_f64(val.latitude)
                        .map(Value::Number)
                        .unwrap_or(Value::Null),
                );
                obj.insert(
                    "longitude".to_string(),
                    Number::from_f64(val.longitude)
                        .map(Value::Number)
                        .unwrap_or(Value::Null),
                );
                Value::Object(obj)
            }
        };

        Ok(result)
    }
}

impl FromFirestoreDocument for Value {
    type Err = FirestoreConversionError;

    fn convert_doc(doc: firestore::Document) -> Result<Value, FirestoreConversionError> {
        let firestore::Document {
            create_time,
            update_time,
            fields,
            name,
        } = doc;

        let mut obj = Map::new();

        let create_time = if let Some(t) = create_time {
            let ms = std::time::Duration::new(t.seconds as u64, t.nanos as u32).as_millis() as u64;
            Value::Number(ms.into())
        } else {
            Value::Null
        };

        let update_time = if let Some(t) = update_time {
            let ms = std::time::Duration::new(t.seconds as u64, t.nanos as u32).as_millis() as u64;
            Value::Number(ms.into())
        } else {
            Value::Null
        };

        obj.insert("create_time".to_string(), create_time);
        obj.insert("update_time".to_string(), update_time);
        obj.insert("name".to_string(), Value::String(name));

        let mut fields_obj = Map::new();
        for (key, val) in fields {
            fields_obj.insert(key.to_string(), Value::convert(val)?);
        }
        obj.insert("fields".to_string(), Value::Object(fields_obj));

        Ok(Value::Object(obj))
    }
}

// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-

impl IntoFirestoreDocumentValue for Value {
    fn into_document_value(self) -> firestore::Value {
        use firestore::value::ValueType;
        let value_type = match self {
            Value::Null => ValueType::NullValue(0),
            Value::Bool(bool) => ValueType::BooleanValue(bool),
            Value::Number(val) => {
                if val.is_f64() {
                    ValueType::DoubleValue(val.as_f64().unwrap_or_default())
                } else {
                    ValueType::IntegerValue(val.as_i64().unwrap_or_default())
                }
            }
            Value::String(val) => ValueType::StringValue(val),
            Value::Array(val) => ValueType::ArrayValue(firestore::ArrayValue {
                values: val
                    .into_iter()
                    .map(|val| val.into_document_value())
                    .collect::<Vec<_>>(),
            }),
            Value::Object(val) => ValueType::MapValue(firestore::MapValue {
                fields: HashMap::from_iter(
                    val.into_iter()
                        .map(|(key, val)| (key, val.into_document_value())),
                ),
            }),
        };

        firestore::Value {
            value_type: Some(value_type),
        }
    }
}

impl IntoFirestoreDocument for Value {
    type Err = FirestoreConversionError;

    /// self are the fields of the document, not the document itself
    fn into_document_from_fields(self) -> Result<firestore::Document, Self::Err> {
        let fields = if let Value::Object(obj) = self {
            HashMap::from_iter(
                obj.into_iter()
                    .map(|(key, val)| (key, val.into_document_value())),
            )
        } else {
            return Err(FirestoreConversionError::IntoFirestoreError(format!(
                "{:?} is not an object",
                self
            )));
        };

        Ok(firestore::Document {
            fields,
            ..Default::default()
        })
    }

    fn into_document(mut self) -> Result<firestore::Document, Self::Err> {
        let name = if let Value::String(name) = self["name"].clone() {
            name
        } else {
            return Err(FirestoreConversionError::IntoFirestoreError(format!(
                "name is not a string"
            )));
        };
        let fields = if let Some(value @ Value::Object(..)) = self.get_mut("fields") {
            value
                .take()
                .into_document_from_fields()
                .map(|doc| doc.fields)?
        } else {
            return Err(FirestoreConversionError::IntoFirestoreError(format!(
                "fields is not an object"
            )));
        };
        let create_time = match self.get("create_time") {
            Some(Value::Number(millis)) if millis.as_u64().is_some() => {
                let millis = millis.as_u64().unwrap();
                let d = std::time::Duration::from_millis(millis);
                Some(Timestamp {
                    seconds: d.as_secs() as i64,
                    nanos: d.subsec_nanos() as i32,
                })
            }
            _ => None,
        };
        let update_time = match self.get("create_time") {
            Some(Value::Number(millis)) if millis.as_u64().is_some() => {
                let millis = millis.as_u64().unwrap();
                let d = std::time::Duration::from_millis(millis);
                Some(Timestamp {
                    seconds: d.as_secs() as i64,
                    nanos: d.subsec_nanos() as i32,
                })
            }
            _ => None,
        };

        Ok(firestore::Document {
            name,
            fields,
            create_time,
            update_time,
        })
    }
}

// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-

pub fn convert_document_fields_to_obj<T>(
    doc: firestore::Document,
) -> Result<T, FirestoreConversionError>
where
    T: DeserializeOwned,
{
    let fields = firestore::MapValue { fields: doc.fields };
    let fields = firestore::Value {
        value_type: Some(firestore::value::ValueType::MapValue(fields)),
    };
    let json = serde_json::Value::convert(fields)?;
    let obj = match serde_json::from_value(json.clone()) {
        Err(err) => {
            tracing::error!(
                "error {} when converting {}",
                err,
                serde_json::to_string_pretty(&json).unwrap(),
            );
            return Err(FirestoreConversionError::FromFirestoreError(format!(
                "error {} when converting document",
                err,
            )));
        }
        Ok(obj) => obj,
    };

    Ok(obj)
}

pub fn convert_document_fields_to_obj_with_id<T>(
    doc: firestore::Document,
) -> Result<T, FirestoreConversionError>
where
    T: DeserializeOwned,
{
    let id = doc.name;
    let mut fields = firestore::MapValue { fields: doc.fields };
    fields.fields.insert(
        "id".to_string(),
        firestore::Value {
            value_type: Some(firestore::value::ValueType::StringValue(id)),
        },
    );
    let fields = firestore::Value {
        value_type: Some(firestore::value::ValueType::MapValue(fields)),
    };
    let json = serde_json::Value::convert(fields)?;
    let obj = serde_json::from_value(json).map_err(|err| {
        // tracing::error!(
        //     "error {} when converting {}",
        //     err,
        //     serde_json::to_string_pretty(&json).unwrap(),
        // );
        tracing::error!("error {err} when converting");
        FirestoreConversionError::FromFirestoreError(format!(
            "error {} when converting document",
            err,
        ))
    })?;

    Ok(obj)
}
