use std::fmt::Display;

use crate::types::column::filename::ShortFilename;
use crate::types::column::identifier::Identifier;

/// [Official documentation](https://learn.microsoft.com/en-us/windows/win32/msi/cabinet)
#[derive(Debug, Clone, PartialEq, derive_more::From, strum::EnumTryAs)]
pub enum CabinetIdentifier {
    Internal(Identifier),
    External(ShortFilename),
}

impl CabinetIdentifier {
    pub fn into_identifier(&self) -> Option<Identifier> {
        self.try_as_internal_ref().cloned()
    }
}

impl Display for CabinetIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CabinetIdentifier::Internal(identifier) => {
                write!(f, "#{identifier}")
            }
            CabinetIdentifier::External(short_filename) => {
                write!(f, "{short_filename}")
            }
        }
    }
}
