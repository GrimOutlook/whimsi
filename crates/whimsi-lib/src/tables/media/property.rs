use crate::types::column::identifier::Identifier;

#[derive(Debug, Clone, PartialEq, derive_more::Display)]
pub enum Property {
    Identifier(Identifier),
    EnvironmentVariable(Identifier),
}

impl From<Property> for msi::Value {
    fn from(value: Property) -> Self {
        value.to_string().into()
    }
}
