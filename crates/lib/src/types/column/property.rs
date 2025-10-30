use crate::types::column::identifier::Identifier;

#[derive(
    Debug, Clone, PartialEq, derive_more::Display, whimsi_macros::StrToValue,
)]

pub enum Property {
    Identifier(Identifier),
    EnvironmentVariable(Identifier),
}
