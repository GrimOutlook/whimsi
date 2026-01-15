use crate::types::helpers::to_msi_value::IntoMsiValue;

pub type Integer = i16;

impl IntoMsiValue for Integer {
    fn into(&self) -> msi::Value {
        self.clone().into()
    }
}
