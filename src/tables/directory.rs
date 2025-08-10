use thiserror::Error;

use crate::types::dao::directory::DirectoryDao;
use crate::types::helpers::directory::{Directory, DirectoryKind, NonRootDirectory, RootDirectory};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct DirectoryTable(Vec<DirectoryDao>);
impl DirectoryTable {
    // Wanted to prevent users from adding more than 1 root directory but this is handled by only
    // implementing From<RootDirectory> for DirectoryTable since the Node enum type only accepts
    // NonRootDirectory. Leaving the input type of `directory` as the wrapper enum `Directory`
    // rather than `NonRootDirectory` so I can use it for adding the RootDirectory instance and
    // it's contents without duplicating the code in the TryFrom implementation.
    //
    fn add(&mut self, directory: &NonRootDirectory, parent: &Directory) -> anyhow::Result<()> {
        self.0.push(DirectoryDao::new(
            // TODO: These clones seem gross. Investigate if these can be removed.
            &directory.clone().into(),
            &parent.clone(),
        )?);
        for child in directory
            .contained()
            .iter()
            .filter_map(|node| node.try_as_directory_ref())
        {
            self.add(&child.borrow(), &directory.clone().into())?;
        }
        Ok(())
    }

    fn add_root(&mut self, root: RootDirectory) -> anyhow::Result<()> {
        todo!()
    }
}

impl TryFrom<RootDirectory> for DirectoryTable {
    type Error = anyhow::Error;

    fn try_from(root_directory: RootDirectory) -> Result<Self, Self::Error> {
        let mut table = DirectoryTable::default();
        table.add_root(root_directory.into())?;
        Ok(table)
    }
}

// TODO: Add error messages
#[derive(Debug, Error)]
pub enum DirectoryTableConversionError {
    #[error("Cannot convert non-root directory to directory table")]
    NonRootDirectory,
}
