//! # Goals
//!
//! Operations I want users to be able to do using directories:
//! - Add files to the directory explicitly.
//! - Add shortcuts to the directory explicitly.
//! - Add a directory to the directory explicitly.
//!     - This is especially needed if the user wants to create empty directories on install.
//! - Convert a `Path` into a valid `Directory` entry.
//!     - This will likely require handling identifiers after everthing is parsed.

use std::cell::RefCell;
use std::fmt::Display;
use std::fs::{self, DirEntry};
use std::option::Option;
use std::path::PathBuf;
use std::rc::Rc;
use std::str::FromStr;

use ambassador::Delegate;
use anyhow::{Context, bail, ensure};
use camino::Utf8PathBuf;
use derivative::Derivative;
use derive_more::{Display, From};
use getset::Getters;
use itertools::Itertools;
use strum::IntoEnumIterator;
use thiserror::Error;

use super::DirectoryError;
use super::kind::DirectoryKind;
use super::kind::ambassador_impl_DirectoryKind;
use super::system_directory::SystemDirectory;
use crate::implement_directory_kind_boilerplate;
use crate::tables::file::helper::File;
use crate::types::column::identifier::Identifier;
use crate::types::helpers::directory_item::DirectoryItem;
use crate::types::helpers::filename::Filename;
use crate::types::properties::system_folder::SystemFolder;

/// All directory information is gathered during the user-input period. No information about
/// directories is generated when traslating to `msi` crate `Package` type.

/// Directory that is a contained within a subdirectory.
///
/// NOTE: The user does not have to create an ID for the directory. The ID for the directory is
/// generated created upon insertion into the `DirectoryTable`.
#[derive(Clone, Debug, Derivative, derive_more::Display, From, Getters, PartialEq)]
#[display("{}", name)]
#[getset(get = "pub")]
#[derivative(PartialOrd, Ord, Eq)]
pub struct Directory {
    #[getset(skip)]
    #[derivative(PartialOrd = "ignore", Ord = "ignore")]
    contained: Vec<DirectoryItem>,

    /// The directory's name (localizable)
    name: Filename,
}

impl DirectoryKind for Directory {
    implement_directory_kind_boilerplate!();

    fn name_conflict(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl FromStr for Directory {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Filename::parse(s)?.into())
    }
}

impl From<Filename> for Directory {
    fn from(value: Filename) -> Self {
        Self {
            contained: Vec::new(),
            name: value,
        }
    }
}

impl Directory {
    /// Checks to see if the name can be found in the system folders. If it can then it returns
    /// a SystemDirectory enum variant. If it can't find it then it uses the `name` field contents
    /// as the `name` of the `SubDirectory` variants wrapped object.
    pub fn new(name: impl ToString) -> anyhow::Result<Self> {
        let name = name.to_string();
        Ok(Directory::from_str(&name)?)
    }
}

impl TryFrom<PathBuf> for Directory {
    type Error = anyhow::Error;
    fn try_from(path: PathBuf) -> anyhow::Result<Self> {
        let valid_entries: Vec<DirEntry> = std::fs::read_dir(&path)?.try_collect()?;
        let mut items: Vec<DirectoryItem> = valid_entries
            .iter()
            .map(|entry| DirectoryItem::try_from(entry.path()))
            // This collection allows me to short circuit when parsing thought the paths if an Err
            // is returned.
            .collect::<anyhow::Result<Vec<DirectoryItem>>>()?
            .into_iter()
            .collect_vec();

        let directory_name = path
            .file_name()
            .ok_or(DirectoryError::NoDirectoryName { path: path.clone() })?
            .to_str()
            .ok_or(DirectoryError::InvalidDirectoryName { path: path.clone() })?;
        // Yes the directory name is stored as a `Filename`. That's just what the datatype is
        // called in the MSI documentation.
        let name = Filename::from_str(directory_name)?;

        Ok(Directory::from(name).with_contents(&mut items))
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use assertables::assert_contains;
    use camino::Utf8PathBuf;

    use crate::{
        tables::directory::{helper::DirectoryKind, system_directory::SystemDirectory},
        types::properties::system_folder::SystemFolder,
    };

    use super::Directory;

    #[test]
    fn add_directory() {
        let mut pf = SystemDirectory::from(SystemFolder::ProgramFilesFolder);
        let man = pf.insert_dir_strict("MAN").unwrap();
        assert_contains!(pf.contents(), &man.clone().into());
        assert_eq!(man.name().to_string(), "MAN");
    }
}
