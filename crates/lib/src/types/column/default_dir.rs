use super::identifier::Identifier;
use crate::types::column::filename::Filename;

#[derive(
    Clone,
    Debug,
    derive_more::Display,
    derive_more::From,
    PartialEq,
    whimsi_macros::IntoStrMsiValue,
)]
pub enum DefaultDir {
    Identifier(Identifier),
    Filename(Filename),
}

impl msi::ToValue for DefaultDir {
    fn to_value(&self) -> msi::Value {
        msi::Value::Str(self.to_string())
    }
}
