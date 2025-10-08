use crate::types::column::identifier::{Identifier, ToIdentifier};

pub trait ToMsiValue {
    fn to_msi_value(&self) -> msi::Value;
}

impl<T: Into<msi::Value> + Clone> ToMsiValue for T {
    fn to_msi_value(&self) -> msi::Value {
        <T as Into<msi::Value>>::into(self.clone())
    }
}

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
