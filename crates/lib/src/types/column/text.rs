use crate::types::helpers::to_msi_value::IntoMsiValue;

pub type Text = String;

impl IntoMsiValue for String {
    fn into(&self) -> msi::Value {
        self.clone().into()
    }
}
