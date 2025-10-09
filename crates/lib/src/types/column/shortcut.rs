use crate::{
    tables::FeatureIdentifier,
    types::column::{formatted::Formatted, identifier::ToIdentifier},
};

/// The Shortcut data type is usually used in the Target column of the Shortcut table. If it
/// contains square brackets ([ ]), the shortcut target is evaluated as a Formatted string.
/// Otherwise, the shortcut is evaluated as an Identifier and must be a valid foreign key into the
/// Feature table.
#[derive(Clone, Debug, PartialEq)]
pub enum Shortcut {
    Formatted(Formatted),
    Identifier(FeatureIdentifier),
}

impl From<Shortcut> for msi::Value {
    fn from(value: Shortcut) -> Self {
        match value {
            Shortcut::Formatted(f) => f.into(),
            Shortcut::Identifier(id) => id.to_identifier().into(),
        }
    }
}
