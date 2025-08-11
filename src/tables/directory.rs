use thiserror::Error;

use crate::types::dao::directory::DirectoryDao;
use crate::types::helpers::directory::{DirectoryKind, SubDirectory, SystemDirectory};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct DirectoryTable(Vec<DirectoryDao>);
impl DirectoryTable {
    fn add_directory_recursive(
        &mut self,
        directory: &SubDirectory,
        parent: &impl DirectoryKind,
    ) -> anyhow::Result<()> {
        self.0.push(DirectoryDao::new(directory, parent)?);
        self.add_children(directory)?;
        Ok(())
    }

    // Root is the only directory that doesn't require a parent
    fn add(&mut self, system_dir: SystemDirectory) -> anyhow::Result<()> {
        self.0.push((&system_dir).try_into()?);
        self.add_children(&system_dir)?;
        Ok(())
    }

    fn add_children(&mut self, directory: &impl DirectoryKind) -> anyhow::Result<()> {
        for child in directory.contained_directories() {
            self.add_directory_recursive(&child.borrow(), directory)?;
        }
        Ok(())
    }
}

// TODO: Add error messages
#[derive(Debug, Error)]
pub enum DirectoryTableConversionError {
    #[error("Cannot convert non-root directory to directory table")]
    NonRootDirectory,
}
