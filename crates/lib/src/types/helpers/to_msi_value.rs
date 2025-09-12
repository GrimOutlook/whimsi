use crate::types::column::identifier::{Identifier, ToIdentifier};

pub trait ToMsiOptionalValue {
    fn to_optional_value(&self) -> whimsi_msi::Value;
}

impl<T: Into<whimsi_msi::Value> + Clone> ToMsiOptionalValue for Option<T> {
    fn to_optional_value(&self) -> whimsi_msi::Value {
        if let Some(val) = self {
            Into::<whimsi_msi::Value>::into(val.clone())
        } else {
            whimsi_msi::Value::Null
        }
    }
}
