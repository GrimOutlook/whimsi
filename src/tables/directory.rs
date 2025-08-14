use std::path::PathBuf;

use thiserror::Error;

use crate::msitable_boilerplate;
use crate::types::dao::directory::DirectoryDao;

use super::MsiBuilderTable;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct DirectoryTable(Vec<DirectoryDao>);
impl MsiBuilderTable for DirectoryTable {
    type TableValue = DirectoryDao;

    // Boilderplate needed to access information on the inner object
    msitable_boilerplate!();

    fn name() -> &'static str {
        "Directory"
    }

    fn init() -> Self {
        todo!()
    }

    fn default_values() -> Vec<Self::TableValue> {
        todo!()
    }
}

impl DirectoryTable {
    pub fn add_directory(&self, directory: DirectoryDao) -> anyhow::Result<()> {
        todo!()
    }
}

// TODO: Add error messages
#[derive(Debug, Error)]
pub enum DirectoryTableConversionError {
    #[error("Cannot convert non-root directory to directory table")]
    NonRootDirectory,
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    // #[test]
    // fn add_directory() {
    //     let msi = Msi::default();
    //     let path = PathBuf::new();
    //     msi.add_pathe(path, SystemFolder::ProgramFiles);
    // }
}
