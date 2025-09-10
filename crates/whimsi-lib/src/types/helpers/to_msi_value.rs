use crate::types::column::identifier::{Identifier, ToIdentifier};

pub trait ToMsiOptionalValue {
    fn to_optional_value(&self) -> msi::Value;
}

impl<T: Into<msi::Value> + Clone> ToMsiOptionalValue for Option<T> {
    fn to_optional_value(&self) -> msi::Value {
        if let Some(val) = self {
            Into::<msi::Value>::into(val.clone())
        } else {
            msi::Value::Null
        }
    }
}
