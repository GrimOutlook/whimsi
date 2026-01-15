use crate::types::column::identifier::Identifier;
use crate::types::helpers::to_msi_value::IntoMsiValue;

#[derive(
    Debug, Clone, PartialEq, derive_more::Display, whimsi_macros::StrToValue,
)]

pub enum Property {
    Identifier(Identifier),
    EnvironmentVariable(Identifier),
}
