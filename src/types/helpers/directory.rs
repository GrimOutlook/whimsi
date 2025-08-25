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
use std::option::Option;
use std::path::PathBuf;
use std::rc::Rc;
use std::str::FromStr;

use ambassador::Delegate;
use anyhow::ensure;
use derive_more::{Display, From};
use getset::Getters;
use itertools::Itertools;
use strum::IntoEnumIterator;
use thiserror::Error;

use crate::types::column::identifier::Identifier;
use crate::types::properties::system_folder::SystemFolder;

use super::filename::Filename;
use super::node::Node;

// TODO: If the `getset` crate ever supports Traits, use them here. I should not have to manually
// make getters just because they are contained in traits.
#[ambassador::delegatable_trait]
pub trait DirectoryKind: Clone {
    fn contained(&self) -> Vec<Node>;
    fn contained_mut(&mut self) -> &mut Vec<Node>;
    fn identifier(&self) -> Option<Identifier>;

    fn contained_directories(&self) -> Vec<Rc<RefCell<SubDirectory>>> {
        self.contained()
            .iter()
            .filter_map(|node| node.try_as_directory_ref())
            .cloned()
            .collect_vec()
    }

    fn insert_dir_strict(&mut self, name: &str) -> anyhow::Result<Rc<RefCell<SubDirectory>>> {
        let new_filename = Filename::parse(name)?;
        self.insert_dir_filename(new_filename)
    }

    fn insert_dir(&mut self, name: &str) -> anyhow::Result<Rc<RefCell<SubDirectory>>> {
        let new_filename = Filename::parse(name)?;
        self.insert_dir_filename(new_filename)
    }

    fn insert_dir_filename(
        &mut self,
        filename: Filename,
    ) -> anyhow::Result<Rc<RefCell<SubDirectory>>> {
        ensure!(
            !self
                .contained()
                .iter()
                .filter_map(|node| node.try_as_directory_ref())
                .any(|dir| dir.borrow().name == filename),
            DirectoryConversionError::DuplicateDirectoryName
        );

        let new_dir = Rc::new(RefCell::new(filename.into()));
        self.contained_mut().push(new_dir.clone().into());
        Ok(new_dir)
    }
}

macro_rules! implement_directory_kind_boilerplate {
    () => {
        fn contained(&self) -> Vec<Node> {
            self.contained.clone()
        }

        fn contained_mut(&mut self) -> &mut Vec<Node> {
            &mut self.contained
        }
    };
}

#[derive(Clone, Debug, Display, PartialEq, Getters)]
#[display("{}", system_folder)]
#[getset(get = "pub")]
pub struct SystemDirectory {
    #[getset(skip)]
    contained: Vec<Node>,
    system_folder: SystemFolder,
}

impl DirectoryKind for SystemDirectory {
    implement_directory_kind_boilerplate!();

    fn identifier(&self) -> Option<Identifier> {
        Some(self.system_folder.into())
    }
}

/// Directory that is a contained within a subdirectory.
///
/// The ID for this directory is created upon insertion into the tables database.
#[derive(Clone, Debug, Display, PartialEq, Getters)]
#[display("{}", name)]
#[getset(get = "pub")]
pub struct SubDirectory {
    #[getset(skip)]
    contained: Vec<Node>,

    id: Option<Identifier>,
    /// The directory's name (localizable)
    name: Filename,
}

impl SubDirectory {
    pub fn new(name: Filename, identifier: Identifier) -> Self {
        Self {
            contained: Vec::new(),
            id: Some(identifier),
            name,
        }
    }
}

impl From<Filename> for SubDirectory {
    fn from(value: Filename) -> Self {
        Self {
            contained: Vec::new(),
            id: None,
            name: value,
        }
    }
}

impl DirectoryKind for SubDirectory {
    implement_directory_kind_boilerplate!();

    fn identifier(&self) -> Option<Identifier> {
        self.id.clone()
    }
}

impl FromStr for SubDirectory {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Filename::parse(s)?.into())
    }
}

#[derive(Clone, Debug, Delegate, Display, From, PartialEq, strum::EnumIs)]
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

    pub fn from_path<P: Into<PathBuf>, I: Into<Identifier>>(
        path: P,
        root_identifier: I,
    ) -> Directory {
        let path = path.into();
        let root_identifier = root_identifier.into();

        // TODO: I can't think of a better way to do this off the top of my head but I am almost
        // certain there is one. Fix this so I don't have to compare it to all the enum values to
        // determine if it is a system folder or not. Type checking doesn't seem like it would work
        // but idk.
        if let Ok(sf) = SystemFolder::try_from(root_identifier) {}
        todo!()
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

#[derive(Debug, Error, From)]
pub enum DirectoryConversionError {
    #[error("Given directory name cannot fit in short filename")]
    DirectoryNameTooLong,
    #[error("Directory name already exists in parent directory")]
    DuplicateDirectoryName,
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use assertables::assert_contains;
    use camino::Utf8PathBuf;

    use crate::types::{
        helpers::directory::DirectoryKind, properties::system_folder::SystemFolder,
    };

    use super::Directory;

    #[test]
    fn add_directory() {
        let mut pf: Directory = SystemFolder::ProgramFiles.into();
        let man = pf.insert_dir_strict("MAN").unwrap();
        assert_contains!(pf.contained(), &man.clone().into());
        assert_eq!(man.borrow().name().to_string(), "MAN");
    }

    #[test]
    fn from_utf8_path() {
        let path = PathBuf::new();
        Directory::from_path(path, SystemFolder::ProgramFiles);
    }
}
