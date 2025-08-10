use std::cell::RefCell;
use std::rc::Rc;
use std::str::FromStr;

use ambassador::Delegate;
use anyhow::ensure;
use derive_more::{Display, From};
use getset::Getters;
use thiserror::Error;

use crate::types::column::identifier::Identifier;
use crate::types::properties::systemfolder::SystemFolder;

use super::filename::Filename;
use super::node::Node;

// TODO: If the `getset` crate ever supports Traits, use them here. I should not have to manually
// make getters just because they are contained in traits.
#[ambassador::delegatable_trait]
pub trait DirectoryKind {
    fn contained(&self) -> Vec<Node>;
    fn contained_mut(&mut self) -> &mut Vec<Node>;
    fn insert_dir(&mut self, name: &str) -> anyhow::Result<Rc<RefCell<NonRootDirectory>>> {
        let new_filename = Filename::parse(name)?;
        self.insert_dir_filename(new_filename)
    }

    fn insert_dir_with_trim(
        &mut self,
        name: &str,
    ) -> anyhow::Result<Rc<RefCell<NonRootDirectory>>> {
        let new_filename = Filename::parse_with_trim(name)?;
        self.insert_dir_filename(new_filename)
    }

    fn insert_dir_filename(
        &mut self,
        filename: Filename,
    ) -> anyhow::Result<Rc<RefCell<NonRootDirectory>>> {
        ensure!(
            !self
                .contained()
                .iter()
                .filter_map(|node| node.try_as_directory_ref())
                .any(|dir| dir.borrow().name == filename),
            DirectoryConversionError::DuplicateDirectoryName
        );

        let wrapped_new_dir = NonRootDirectory::new(filename);
        let new_dir = Rc::new(RefCell::new(wrapped_new_dir));
        self.contained_mut().push(new_dir.clone().into());
        Ok(new_dir)
    }
}
macro_rules! implement_directory_kind_simple {
    ($struct_name:ty) => {
        impl DirectoryKind for $struct_name {
            fn contained(&self) -> Vec<Node> {
                self.contained.clone()
            }

            fn contained_mut(&mut self) -> &mut Vec<Node> {
                &mut self.contained
            }
        }
    };
}

#[derive(Clone, Debug, Display, PartialEq, Getters)]
#[display("{}", id)]
#[getset(get = "pub")]
pub struct RootDirectory {
    #[getset(skip)]
    contained: Vec<Node>,

    /// ID of this directory. This is always `TARGETDIR`.
    id: Identifier,
    /// Identifier for the root directory. This is always `SourceDir`.
    name: Identifier,
}

impl RootDirectory {
    pub fn insert_system_folder(
        &mut self,
        system_folder: SystemFolder,
    ) -> Rc<RefCell<NonRootDirectory>> {
        let new_dir = Rc::new(RefCell::new(NonRootDirectory::system_folder(system_folder)));
        self.contained.push(new_dir.clone().into());
        new_dir
    }
}

implement_directory_kind_simple!(RootDirectory);

/// Directory that is a contained within a subdirectory.
///
/// The ID for this directory is created upon insertion into the tables database.
#[derive(Clone, Debug, Display, PartialEq, Getters)]
#[display("{}", name)]
#[getset(get = "pub")]
pub struct NonRootDirectory {
    #[getset(skip)]
    contained: Vec<Node>,

    /// This can either be a system directory or None. If None, this folder
    /// will not be parsed as a system folder and instead will have an identifier randomly
    /// generated for it when placing into the MSI database. Otherwise it will use the system
    /// folder ID as the identifier in the DAO.
    id: Option<SystemFolder>,
    /// The directory's name (localizable)
    name: Filename,
}

impl NonRootDirectory {
    pub fn new(name: Filename) -> Self {
        Self {
            contained: Vec::new(),
            id: None,
            name,
        }
    }

    pub fn system_folder(system_folder: SystemFolder) -> Self {
        Self {
            contained: Vec::new(),
            id: Some(system_folder),
            name: Filename::parse(".").unwrap(),
        }
    }
}

implement_directory_kind_simple!(NonRootDirectory);

impl FromStr for NonRootDirectory {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(Filename::parse(s)?))
    }
}

#[derive(Clone, Debug, Delegate, Display, From, PartialEq, strum::EnumIs)]
#[delegate(DirectoryKind)]
pub enum Directory {
    RootDirectory(RootDirectory),
    NonRootDirectory(NonRootDirectory),
}

impl Directory {
    /// Create a new root directory.
    pub fn root() -> RootDirectory {
        RootDirectory {
            contained: Vec::new(),
            id: Identifier::from_str("TARGETDIR").expect("Default root directory caused panic"),
            name: Identifier::from_str("SourceDir").expect("Default root dir caused panic"),
        }
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
    use assertables::assert_contains;

    use crate::types::{helpers::directory::DirectoryKind, properties::systemfolder::SystemFolder};

    use super::Directory;

    #[test]
    fn add_directory() {
        let mut root = Directory::root();
        let pf = root.insert_system_folder(SystemFolder::PROGRAMFILES);
        assert_contains!(root.contained(), &pf.clone().into());
        let man = (*pf.borrow_mut()).insert_dir("MAN").unwrap();
        assert_contains!(pf.borrow().contained(), &man.clone().into());
        assert_eq!(man.borrow().name().to_string(), "MAN");
    }
}
