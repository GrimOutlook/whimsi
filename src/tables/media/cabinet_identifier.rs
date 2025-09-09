use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;
use std::str::FromStr;

use anyhow::{bail, ensure};

use crate::types::column::filename::ShortFilename;
use crate::types::column::identifier::{Identifier, ToOptionalIdentifier};
use crate::{define_identifier_generator, define_specific_identifier};

define_specific_identifier!(cabinet);

/// [Official documentation](https://learn.microsoft.com/en-us/windows/win32/msi/cabinet)
#[derive(Debug, Clone, PartialEq, derive_more::From, strum::EnumTryAs)]
pub enum CabinetHandle {
    Internal(CabinetIdentifier),
    External(ShortFilename),
}

impl FromStr for CabinetIdentifier {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        Ok(CabinetIdentifier(Identifier::from_str(s)?))
    }
}

impl Display for CabinetHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CabinetHandle::Internal(identifier) => {
                write!(f, "#{identifier}")
            }
            CabinetHandle::External(short_filename) => {
                write!(f, "{short_filename}")
            }
        }
    }
}

define_identifier_generator!(cabinet);
