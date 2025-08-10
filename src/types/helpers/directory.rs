use std::cell::RefCell;
use std::rc::Rc;
use std::str::FromStr;

use ambassador::Delegate;
use derive_more::{Display, From};

use crate::tables::directory::ambassador_impl_Container;

use crate::{
    tables::directory::Container,
    types::column::{filename::MsiFilename, identifier::Identifier},
};

use super::filename::Filename;
use super::node::Node;

#[derive(Clone, Debug, Display, PartialEq)]
#[display("{}", directory)]
pub struct RootDirectory {
    contained: Vec<Node>,
    /// ID of this directory
    directory: Identifier,
    /// Identifier for the root directory. This is always `SourceDir`.
    name: Identifier,
}

impl Container for RootDirectory {
    fn contained(&self) -> Vec<Node> {
        self.contained.clone()
    }

    fn contained_mut(&mut self) -> &mut Vec<Node> {
        &mut self.contained
    }
}

/// Directory that is a contained within a subdirectory.
///
/// The ID for this directory is created upon insertion into the tables database.
#[derive(Clone, Debug, Display, PartialEq)]
#[display("{}", name)]
pub struct NonRootDirectory {
    contained: Vec<Node>,
    /// The directory's name (localizable)
    name: Filename,
}

impl NonRootDirectory {
    fn new(name: Filename) -> Self {
        Self {
            contained: Vec::new(),
            name,
        }
    }

    // TODO: Deduplicate this from the `Directory` enum implementation
    pub fn insert_dir(&mut self, directory: &str) -> anyhow::Result<Rc<RefCell<NonRootDirectory>>> {
        // TODO: Check if this directory clashes with one that's already contained.
        let new_dir = Rc::new(RefCell::new(NonRootDirectory::from_str(directory)?));
        self.contained_mut().push(new_dir.clone().into());
        Ok(new_dir.into())
    }
}

impl Container for NonRootDirectory {
    fn contained(&self) -> Vec<Node> {
        self.contained.clone()
    }
    fn contained_mut(&mut self) -> &mut Vec<Node> {
        &mut self.contained
    }
}

impl FromStr for NonRootDirectory {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(Filename::parse(s)?))
    }
}

#[derive(Clone, Debug, Delegate, Display, From, PartialEq, strum::EnumIs)]
#[delegate(Container)]
pub enum Directory {
    RootDirectory(RootDirectory),
    NonRootDirectory(NonRootDirectory),
}

impl Directory {
    /// Create a new root directory.
    pub fn root() -> Directory {
        RootDirectory {
            contained: Vec::new(),
            directory: Identifier::from_str("TARGETDIR")
                .expect("Default root directory caused panic"),
            name: Identifier::from_str("SourceDir").expect("Default root dir caused panic"),
        }
        .into()
    }
    pub fn insert_dir(&mut self, directory: &str) -> anyhow::Result<Rc<RefCell<NonRootDirectory>>> {
        // TODO: Check if this directory clashes with one that's already contained.
        let new_dir = Rc::new(RefCell::new(NonRootDirectory::from_str(directory)?));
        self.contained_mut().push(new_dir.clone().into());
        Ok(new_dir.into())
    }
}

pub enum DirectoryConversionError {
    DirectoryNameTooLong { shortened: Directory },
}

#[cfg(test)]
mod test {
    use assertables::assert_contains;

    use crate::tables::directory::Container;

    use super::Directory;

    #[test]
    fn add_directory() {
        let mut root = Directory::root();
        let pf = root.insert_dir("PFiles").unwrap();
        assert_contains!(root.contained(), &pf.clone().into());
        let man = (*pf.borrow_mut()).insert_dir("MAN").unwrap();
        assert_contains!(pf.borrow().contained(), &man.into());
        // assert_eq!(man.get())
    }
}
