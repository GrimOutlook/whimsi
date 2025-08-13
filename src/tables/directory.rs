use std::path::PathBuf;

use camino::Utf8PathBuf;
use thiserror::Error;

use crate::types::column::identifier::Identifier;
use crate::types::dao::directory::DirectoryDao;
use crate::{Msi, implement_boilerplate_table_kind};

use super::TableKind;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct DirectoryTable(Vec<DirectoryDao>);
implement_boilerplate_table_kind!(DirectoryTable);

impl Msi {
    /// Insert a given filesystem path into the directories table recursively.
    ///
    /// ## Arguments
    ///
    /// - *path* Path to the items you want to be copied to the system on install.
    /// - *install_path_identifier* `Identifier` for the directory where the given path should be
    /// placed. Identifer should already be present in the `Directory` table or should be a
    /// `SystemFolder`.
    ///
    /// ## Example
    ///
    /// ```
    /// # use whimsi_lib::Msi;
    /// # use whimsi_lib::tables::TableKind;
    /// # use whimsi_lib::types::properties::system_folder::SystemFolder;
    ///
    /// # use assert_fs::TempDir;
    /// # use assert_fs::fixture::PathChild;
    /// # use assert_fs::fixture::PathCreateDir;
    /// # use assert_fs::fixture::FileTouch;
    ///
    /// # let temp_dir = TempDir::new().unwrap();
    /// # let child_dir1 = temp_dir.child("child_dir1");
    /// # child_dir1.create_dir_all().unwrap();
    /// # let child_dir2 = temp_dir.child("child_dir2");
    /// # child_dir2.create_dir_all().unwrap();
    /// # let file_1 = temp_dir.child("file1.txt");
    /// # file_1.touch().unwrap();
    /// # let file_2 = child_dir1.child("file2.pdf");
    /// # file_2.touch().unwrap();
    /// // path/to/temp_dir/
    /// // | - file1.txt
    /// // | child_dir1/
    /// // | child_dir2/
    /// //   | - file2.pdf
    /// // With a file system that looks like the above and using ProgramFiles for the
    /// // install_path_identifier
    ///
    /// let msi = Msi::default();
    /// msi.add_path(temp_dir.path(), SystemFolder::ProgramFiles);
    ///
    /// // You will end up with the following on the windows install.
    /// // C:/ProgramFiles/
    /// // | - file1.txt
    /// // | child_dir1/
    /// // | child_dir2/
    /// //   | - file2.pdf
    ///
    /// let table = msi.table(TableKind::Directories).unwrap();
    /// // 1 entry for each file/directory. 1 entry for ProgramFiles. 1 entry for root directory
    /// // that is always required.
    /// assert_eq!(table.len(), 6);
    /// ```
    pub fn add_path<T: Into<PathBuf>, P: Into<Identifier>>(
        &mut self,
        path: T,
        install_path_identifier: P,
    ) -> anyhow::Result<()> {
        let mut table: DirectoryTable = self.table_or_new(TableKind::Directories).try_into()?;
        let path = Utf8PathBuf::from_path_buf(path.into());
        // TODO: Create a directory table if it doesn't exist.
        // TODO: Add the directory structure recursively.
        //
        // table.0.push(DirectoryDao::new(directory, parent)?);
        // self.add_children(directory)?;
        Ok(())
    }

    // // Root is the only directory that doesn't require a parent
    // fn add(&mut self, system_dir: SystemDirectory) -> anyhow::Result<()> {
    //     let mut table: DirectoryTable = self.table(TableKind::Directories).unwrap().try_into()?;
    //     table.0.push((&system_dir).try_into()?);
    //     self.add_children(&system_dir)?;
    //     Ok(())
    // }
    //
    // fn add_children(&mut self, directory: &impl DirectoryKind) -> anyhow::Result<()> {
    //     for child in directory.contained_directories() {
    //         self.add_directory_recursive(&child.borrow(), directory)?;
    //     }
    //     Ok(())
    // }
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
