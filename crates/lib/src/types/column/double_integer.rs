use crate::types::helpers::to_msi_value::IntoMsiValue;

pub type DoubleInteger = i32;

impl IntoMsiValue for DoubleInteger {
    fn into(&self) -> msi::Value {
        self.clone().into()
    }
}
