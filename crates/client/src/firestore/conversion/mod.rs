use anyhow::Result;
use chrono::{DateTime, Utc};
use firestore_grpc::v1::{self as firestore, value::ValueType};
use std::fmt::{Debug, Display};

pub mod json;
pub use json::*;

pub trait FromFirestoreDocument: Sized {
    type Err: Debug + Display;

    fn convert_doc(doc: firestore::Document) -> Result<Self, Self::Err>;
}

pub trait FromFirestoreValue: Sized {
    type Err: Debug + Display;

    fn convert(val: firestore::Value) -> Result<Self, Self::Err>;
}

pub trait IntoFirestoreDocument {
    type Err: Debug + Display;
    fn into_document_from_fields(self) -> Result<firestore::Document, Self::Err>;
    fn into_document(self) -> Result<firestore::Document, Self::Err>;
}

impl IntoFirestoreDocument for firestore::Document {
    type Err = anyhow::Error;

    fn into_document_from_fields(self) -> Result<Self> {
        Ok(self)
    }

    fn into_document(self) -> Result<Self> {
        Ok(self)
    }
}

pub trait IntoFirestoreDocumentValue {
    fn into_document_value(self) -> firestore::Value;
}

impl IntoFirestoreDocumentValue for firestore::Value {
    fn into_document_value(self) -> Self {
        self
    }
}

impl<T> IntoFirestoreDocumentValue for Vec<T>
where
    T: IntoFirestoreDocumentValue,
{
    fn into_document_value(self) -> firestore::Value {
        firestore::Value {
            value_type: Some(ValueType::ArrayValue(firestore::ArrayValue {
                values: self
                    .into_iter()
                    .map(|ea| ea.into_document_value())
                    .collect(),
            })),
        }
    }
}

impl<T> IntoFirestoreDocumentValue for &[T]
where
    T: IntoFirestoreDocumentValue + Clone,
{
    fn into_document_value(self) -> firestore::Value {
        firestore::Value {
            value_type: Some(ValueType::ArrayValue(firestore::ArrayValue {
                values: self
                    .iter()
                    .map(|ea| ea.clone().into_document_value())
                    .collect(),
            })),
        }
    }
}

impl<T> IntoFirestoreDocumentValue for Option<T>
where
    T: IntoFirestoreDocumentValue,
{
    fn into_document_value(self) -> firestore::Value {
        match self {
            Some(val) => val.into_document_value(),
            None => firestore::Value { value_type: None },
        }
    }
}

impl<T> IntoFirestoreDocumentValue for &T
where
    T: IntoFirestoreDocumentValue + Clone,
{
    fn into_document_value(self) -> firestore::Value {
        self.clone().into_document_value()
    }
}

impl IntoFirestoreDocumentValue for String {
    fn into_document_value(self) -> firestore::Value {
        firestore::Value {
            value_type: Some(firestore::value::ValueType::StringValue(self)),
        }
    }
}

impl IntoFirestoreDocumentValue for i32 {
    fn into_document_value(self) -> firestore::Value {
        firestore::Value {
            value_type: Some(firestore::value::ValueType::IntegerValue(self.into())),
        }
    }
}

impl IntoFirestoreDocumentValue for i64 {
    fn into_document_value(self) -> firestore::Value {
        firestore::Value {
            value_type: Some(firestore::value::ValueType::IntegerValue(self)),
        }
    }
}

impl IntoFirestoreDocumentValue for u32 {
    fn into_document_value(self) -> firestore::Value {
        firestore::Value {
            value_type: Some(firestore::value::ValueType::IntegerValue(self.into())),
        }
    }
}

impl IntoFirestoreDocumentValue for f64 {
    fn into_document_value(self) -> firestore::Value {
        firestore::Value {
            value_type: Some(firestore::value::ValueType::DoubleValue(self)),
        }
    }
}

impl IntoFirestoreDocumentValue for DateTime<Utc> {
    fn into_document_value(self) -> firestore::Value {
        let time = std::time::UNIX_EPOCH
            + std::time::Duration::from_millis(self.timestamp_millis().try_into().unwrap());
        firestore_grpc::v1::Value {
            value_type: Some(firestore_grpc::v1::value::ValueType::TimestampValue(
                time.into(),
            )),
        }
    }
}
