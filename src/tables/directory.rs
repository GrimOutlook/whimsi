use thiserror::Error;

use crate::types::dao::directory::DirectoryDao;
use crate::types::helpers::directory::{DirectoryKind, NonRootDirectory, RootDirectory};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct DirectoryTable(Vec<DirectoryDao>);
impl DirectoryTable {
    fn add_directory_recursive(
        &mut self,
        directory: &NonRootDirectory,
        parent: &impl DirectoryKind,
    ) -> anyhow::Result<()> {
        self.0.push(DirectoryDao::new(directory, parent)?);
        self.add_children(directory)?;
        Ok(())
    }

    // Root is the only directory that doesn't require a parent
    fn add_root(&mut self, root: RootDirectory) -> anyhow::Result<()> {
        self.0.push((&root).into());
        self.add_children(&root)?;
        Ok(())
    }

    fn add_children(&mut self, directory: &impl DirectoryKind) -> anyhow::Result<()> {
        for child in directory.contained_directories() {
            self.add_directory_recursive(&child.borrow(), directory)?;
        }
        Ok(())
    }
}

impl TryFrom<RootDirectory> for DirectoryTable {
    type Error = anyhow::Error;

    fn try_from(root_directory: RootDirectory) -> Result<Self, Self::Error> {
        let mut table = DirectoryTable::default();
        table.add_root(root_directory)?;
        Ok(table)
    }
}

// TODO: Add error messages
#[derive(Debug, Error)]
pub enum DirectoryTableConversionError {
    #[error("Cannot convert non-root directory to directory table")]
    NonRootDirectory,
}
