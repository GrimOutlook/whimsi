use crate::types::column::identifier::Identifier;

#[derive(Debug, Clone, PartialEq, derive_more::Display)]
pub enum Property {
    Identifier(Identifier),
    EnvironmentVariable(Identifier),
}

impl msi::ToValue for Property {
    fn to_value(&self) -> msi::Value {
        self.to_string().into()
    }
}
