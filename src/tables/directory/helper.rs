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

use super::kind::DirectoryKind;
use super::kind::ambassador_impl_DirectoryKind;
use super::subdirectory::SubDirectory;
use super::system_directory::SystemDirectory;
use crate::tables::file::helper::File;
use crate::types::column::identifier::Identifier;
use crate::types::helpers::directory_item::DirectoryItem;
use crate::types::helpers::filename::Filename;
use crate::types::properties::system_folder::SystemFolder;

/// All directory information is gathered during the user-input period. No information about
/// directories is generated when traslating to `msi` crate `Package` type.
#[derive(
    Delegate,
    Clone,
    Debug,
    Display,
    Eq,
    From,
    Ord,
    PartialEq,
    PartialOrd,
    strum::EnumIs,
    strum::EnumTryAs,
)]
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

        let val = if let Ok(id) = Identifier::from_str(&name)
            && let Ok(sf) = SystemFolder::from_identifier(&id)
        {
            sf.into()
        } else {
            let subdir = SubDirectory::from_str(&name)?;
            subdir.into()
        };

        Ok(val)
    }

    pub fn conflict(&self, other: &Self) -> bool {
        if let Some(source) = self.try_as_system_directory_ref()
            && let Some(target) = other.try_as_system_directory_ref()
        {
            source.system_folder() == target.system_folder()
        } else if let Some(source) = self.try_as_sub_directory_ref()
            && let Some(target) = other.try_as_sub_directory_ref()
        {
            source.name() == target.name()
        } else {
            false
        }
    }

    pub fn from_system_folder(value: SystemFolder) -> Self {
        Directory::from(value)
    }

    pub fn print_structure(&self) {
        self.print_content_structure(0)
    }

    fn print_content_structure(&self, depth: usize) {
        let delimiter = "|- ";
        let depth_str = |x| " ".repeat(x * delimiter.len());
        if depth == 0 {
            println!("{self}/");
        } else {
            println!("{}{delimiter}{self}/", depth_str(depth))
        }
        let files = self.contained_files().into_iter().sorted();
        let directories = self.contained_directories().into_iter().sorted();
        for file in files {
            println!("{}{delimiter}{file}", depth_str(depth + 1));
        }
        for directory in directories {
            directory.print_content_structure(depth + 1);
        }
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

        let mut subdir: SubDirectory = path.to_path_buf().try_into()?;
        subdir.add_contents(&mut items);
        Ok(Directory::SubDirectory(subdir))
    }
}

impl From<SystemFolder> for Directory {
    fn from(value: SystemFolder) -> Self {
        let sd = SystemDirectory::from(value);
        Directory::from(sd)
    }
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
        let mut pf: Directory = SystemFolder::ProgramFilesFolder.into();
        let man = pf.insert_dir_strict("MAN").unwrap();
        assert_contains!(pf.contents(), &man.clone().into());
        assert_eq!(
            man.try_as_sub_directory().unwrap().name().to_string(),
            "MAN"
        );
    }
}
