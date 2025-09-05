use crate::types::column::identifier::Identifier;

#[derive(Debug, Clone, PartialEq)]
pub enum Property {
    Identifier(Identifier),
    EnvironmentVariable(Identifier),
}
