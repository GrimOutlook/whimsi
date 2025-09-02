use std::{path::PathBuf, str::FromStr};

use derivative::Derivative;
use getset::Getters;

use crate::{
    implement_directory_kind_boilerplate,
    types::helpers::{directory_item::DirectoryItem, filename::Filename},
};

use super::{DirectoryError, kind::DirectoryKind};

/// Directory that is a contained within a subdirectory.
///
/// NOTE: The user does not have to create an ID for the directory. The ID for the directory is
/// generated created upon insertion into the `DirectoryTable`.
#[derive(Clone, Debug, derive_more::Display, PartialEq, Getters, Derivative)]
#[display("{}", name)]
#[getset(get = "pub")]
#[derivative(PartialOrd, Ord, Eq)]
pub struct SubDirectory {
    #[getset(skip)]
    #[derivative(PartialOrd = "ignore", Ord = "ignore")]
    contained: Vec<DirectoryItem>,

    /// The directory's name (localizable)
    name: Filename,
}

impl DirectoryKind for SubDirectory {
    implement_directory_kind_boilerplate!();
}

impl FromStr for SubDirectory {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Filename::parse(s)?.into())
    }
}

impl From<Filename> for SubDirectory {
    fn from(value: Filename) -> Self {
        Self {
            contained: Vec::new(),
            name: value,
        }
    }
}

impl TryFrom<PathBuf> for SubDirectory {
    type Error = anyhow::Error;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        let path: PathBuf = value.into();
        let directory_name = path
            .file_name()
            .ok_or(DirectoryError::NoDirectoryName { path: path.clone() })?
            .to_str()
            .ok_or(DirectoryError::InvalidDirectoryName { path: path.clone() })?;
        // Yes the directory name is stored as a `Filename`. That's just what the datatype is
        // called in the MSI documentation.
        let name = Filename::from_str(directory_name)?;
        Ok(SubDirectory::from(name))
    }
}
