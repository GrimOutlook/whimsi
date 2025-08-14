// This is basically an internal representation of what is contained in the final MSI, just in more
// manipulatable data types.
//
// Properties are derived from this table:
// https://learn.microsoft.com/en-us/windows/win32/msi/database-tables
// This is basically an internal representation of what is contained in the final MSI, just in more
// manipulatable data types.
//
// Properties are derived from this table:
// https://learn.microsoft.com/en-us/windows/win32/msi/database-tables
// TODO: Figure out why this causes tests to not run.
// TODO: Figure out why this causes the rust-lsp to break and tests to not run.
// #![cfg(not(debug_assertions))]
// #![deny(
//     clippy::all,
//     missing_docs,
//     missing_debug_implementations,
//     rustdoc::all,
//     unsafe_code
// )]
#![cfg(debug_assertions)]
#![allow(dead_code)]

pub mod constants;
pub mod tables;
pub mod types;

use std::{collections::HashMap, path::PathBuf};

use camino::Utf8PathBuf;
use getset::Getters;
use tables::MsiBuilderTables;
use types::column::{ColumnValue, identifier::Identifier};

type Identifiers = HashMap<Identifier, ColumnValue>;

/// An in-memory representation of the final MSI to be created.
#[derive(Default, Getters)]
#[getset(get = "pub")]
pub struct MsiBuilder {
    /// Tracks identifiers used to relate items between tables.
    identifiers: Identifiers,
    tables: MsiBuilderTables,
}

impl MsiBuilder {
    /// Insert a given filesystem path's contents into the Msi for installation.
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
    /// # use whimsi_lib::MsiBuilder;
    /// # use whimsi_lib::tables::MsiBuilderTable;
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
    /// let mut msi = MsiBuilder::default();
    /// msi.add_path(temp_dir.path(), SystemFolder::ProgramFiles);
    ///
    /// // You will end up with the following on the windows install.
    /// // C:/ProgramFiles/
    /// // | - file1.txt
    /// // | child_dir1/
    /// // | child_dir2/
    /// //   | - file2.pdf
    ///
    /// let table = msi.tables().directory();
    /// // 1 entry for each *directory*. 1 entry for ProgramFiles. 1 entry for root directory
    /// // that is always required.
    /// assert_eq!(table.len(), 4);
    /// ```
    pub fn add_path<T: Into<PathBuf>, P: Into<Identifier>>(
        &mut self,
        path: T,
        install_path_identifier: P,
    ) -> anyhow::Result<()> {
        let table = self.tables.directory();
        let path = path.into();
        // TODO: We will handle files later. Just working on directories for now.
        //
        // If the path is not a directory, don't add it to the directories table.
        if path.is_dir() {
            table.add_directory(path.try_into()?)?;
        }

        Ok(())
    }
}
