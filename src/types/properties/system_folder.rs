// TODO: Look into enabling custom directories based on properties.
// I did about an hour of work before realizing it was more effort than I needed to use right now
// while just trying to get basic functionality up and running. From the little work I did I
// learned custom system folders will need to track their own parent, as it can either be None
// (meaning TARGETDIR) or it could be another directory already defined. It will also have to be
// verified that the identifier given for the parent (if not None) and the ID given for the new
// custom system directory is in the `Property` table beforehand as this is where the value for the
// directory Identifier will come from.

use itertools::Itertools;
use strum::IntoEnumIterator;
use thiserror::Error;

use crate::types::column::identifier::{Identifier, ToIdentifier};

#[derive(Clone, Copy, Debug, PartialEq, strum::Display, strum::EnumIter)]
pub enum SystemFolder {
    TARGETDIR,
    ProgramFiles,
}

impl PartialEq<Identifier> for SystemFolder {
    fn eq(&self, other: &Identifier) -> bool {
        other == &self.into()
    }
}

impl TryFrom<Identifier> for SystemFolder {
    type Error = anyhow::Error;

    fn try_from(identifier: Identifier) -> Result<Self, Self::Error> {
        SystemFolder::iter()
            .find(|f| identifier == f.into())
            .ok_or(SystemFolderConversionError::InvalidSystemFolder { identifier }.into())
    }
}

impl ToIdentifier for SystemFolder {
    fn to_identifier(&self) -> Identifier {
        self.into()
    }
}

#[derive(Debug, Error)]
pub enum SystemFolderConversionError {
    #[error("Identifer {identifier} didn't match any known system folder")]
    InvalidSystemFolder { identifier: Identifier },
}
