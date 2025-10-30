use crate::tables::FeatureIdentifier;
use crate::types::column::formatted::Formatted;
use crate::types::column::identifier::ToIdentifier;

/// The Shortcut data type is usually used in the Target column of the Shortcut
/// table. If it contains square brackets ([ ]), the shortcut target is
/// evaluated as a Formatted string. Otherwise, the shortcut is evaluated as an
/// Identifier and must be a valid foreign key into the Feature table.
#[derive(
    Clone, Debug, PartialEq, strum::Display, whimsi_macros::StrToValue,
)]
pub enum Shortcut {
    Formatted(Formatted),
    Identifier(FeatureIdentifier),
}
