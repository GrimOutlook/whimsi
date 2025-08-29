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
use std::fs::{self, DirEntry};
use std::option::Option;
use std::path::PathBuf;
use std::rc::Rc;
use std::str::FromStr;

use ambassador::Delegate;
use anyhow::{Context, bail, ensure};
use camino::Utf8PathBuf;
use derive_more::{Display, From};
use getset::Getters;
use itertools::Itertools;
use strum::IntoEnumIterator;
use thiserror::Error;

use crate::types::column::identifier::Identifier;
use crate::types::helpers::directory_item::DirectoryItem;
use crate::types::helpers::filename::Filename;
use crate::types::properties::system_folder::SystemFolder;

// TODO: If the `getset` crate ever supports Traits, use them here. I should not have to manually
// make getters just because they are contained in traits.
#[ambassador::delegatable_trait]
pub trait DirectoryKind: Clone {
    fn contents(&self) -> Vec<DirectoryItem>;
    fn contents_mut(&mut self) -> &mut Vec<DirectoryItem>;

    fn contained_directories(&self) -> Vec<Directory> {
        self.contents()
            .iter()
            .filter_map(|node| node.try_as_directory_ref())
            .cloned()
            .collect_vec()
    }

    fn insert_dir_strict(&mut self, name: &str) -> anyhow::Result<Directory> {
        let new_filename = Filename::parse(name)?;
        self.insert_dir_filename(new_filename)
    }

    fn insert_dir(&mut self, name: &str) -> anyhow::Result<Directory> {
        let new_filename = Filename::parse(name)?;
        self.insert_dir_filename(new_filename)
    }

    fn insert_dir_filename(&mut self, filename: Filename) -> anyhow::Result<Directory> {
        let contents = self.contents();
        let contained_dirs = contents
            .iter()
            .filter_map(|node| node.try_as_directory_ref());
        ensure!(
            !contained_dirs
                .filter_map(|directory| directory.try_as_sub_directory_ref())
                .any(|dir| *dir.name() == filename),
            DirectoryConversionError::DuplicateDirectory {
                name: filename.to_string()
            }
        );

        let new_dir = Directory::SubDirectory(filename.into());
        self.contents_mut().push(new_dir.clone().into());
        Ok(new_dir)
    }
}

macro_rules! implement_directory_kind_boilerplate {
    () => {
        fn contents(&self) -> Vec<DirectoryItem> {
            self.contained.clone()
        }

        fn contents_mut(&mut self) -> &mut Vec<DirectoryItem> {
            &mut self.contained
        }
    };
}

#[derive(Clone, Debug, Display, PartialEq, Getters)]
#[display("{}", system_folder)]
#[getset(get = "pub")]
pub struct SystemDirectory {
    #[getset(skip)]
    contained: Vec<DirectoryItem>,
    system_folder: SystemFolder,
}

impl DirectoryKind for SystemDirectory {
    implement_directory_kind_boilerplate!();
}

/// Directory that is a contained within a subdirectory.
///
/// NOTE: The user does not have to create an ID for the directory. The ID for the directory is
/// generated created upon insertion into the `DirectoryTable`.
#[derive(Clone, Debug, Display, PartialEq, Getters)]
#[display("{}", name)]
#[getset(get = "pub")]
pub struct SubDirectory {
    #[getset(skip)]
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
        path.file_name()
            .ok_or(DirectoryConversionError::NoDirectoryName { path: path.clone() })?
            .to_str()
            .ok_or(DirectoryConversionError::InvalidDirectoryName { path: path.clone() })?
            .parse()
            .into()
    }
}

#[derive(Clone, Debug, Delegate, Display, From, PartialEq, strum::EnumIs, strum::EnumTryAs)]
#[delegate(DirectoryKind)]
pub enum Directory {
    SystemDirectory(SystemDirectory),
    SubDirectory(SubDirectory),
}

impl Directory {
    /// Checks to see if the name can be found in the system folders. If it can then it returns
    /// a SystemDirectory enum variant. If it can't find it then it uses the `name` field contents
    /// as the `name` of the `SubDirectory` variants wrapped object.
    pub fn new(name: impl ToString) -> anyhow::Result<Self> {
        let name = name.to_string();

        let val = if let Ok(id) = name.parse::<Identifier>()
            && let Ok(sf) = TryInto::<SystemFolder>::try_into(id)
        {
            sf.into()
        } else {
            let subdir = name.parse::<SubDirectory>()?;
            subdir.into()
        };

        Ok(val)
    }

    fn add_contents(mut self, mut contents: Vec<DirectoryItem>) -> Self {
        self.contents_mut().append(&mut contents);
        self
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

        let subdir = path.to_path_buf().try_into()?;
        Ok(Directory::SubDirectory(subdir).add_contents(items))
    }
}

impl From<SystemFolder> for Directory {
    fn from(value: SystemFolder) -> Self {
        SystemDirectory {
            contained: Vec::new(),
            system_folder: value,
        }
        .into()
    }
}

#[derive(Debug, Error)]
pub enum DirectoryConversionError {
    #[error("Given directory name [{name}] cannot fit in short filename")]
    DirectoryNameTooLong { name: String },
    #[error("Directory name [{name}] already exists in parent directory")]
    DuplicateDirectory { name: String },
    #[error("No directory name could be found for path [{path}]")]
    NoDirectoryName { path: PathBuf },
    #[error("Invalid directory name found for path [{path}]")]
    InvalidDirectoryName { path: PathBuf },
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use assertables::assert_contains;
    use camino::Utf8PathBuf;

    use crate::{
        tables::directory::helper::DirectoryKind, types::properties::system_folder::SystemFolder,
    };

    use super::Directory;

    #[test]
    fn add_directory() {
        let mut pf: Directory = SystemFolder::ProgramFiles.into();
        let man = pf.insert_dir_strict("MAN").unwrap();
        assert_contains!(pf.contents(), &man.clone().into());
        assert_eq!(
            man.try_as_sub_directory().unwrap().name().to_string(),
            "MAN"
        );
    }
}
