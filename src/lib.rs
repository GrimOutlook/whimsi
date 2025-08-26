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
#![allow(warnings)]

pub mod constants;
pub mod tables;
pub mod types;

use std::{collections::HashMap, path::PathBuf, process::id, str::FromStr};

use anyhow::{bail, ensure};
use getset::Getters;
use rand::distr::{Alphanumeric, SampleString};
use tables::{MsiBuilderTables, TableEntry};
use thiserror::Error;
use types::{
    column::{ColumnValue, identifier::Identifier},
    dao::directory::DirectoryDao,
    helpers::directory::{Directory, DirectoryKind, SubDirectory, SystemDirectory},
    properties::system_folder::SystemFolder,
};
type Identifiers = HashMap<Identifier, TableEntry>;

/// An in-memory representation of the final MSI to be created.
#[derive(Debug, Default, Getters)]
#[getset(get = "pub")]
pub struct MsiBuilder {
    /// Tracks identifiers used to relate items between tables.
    #[getset(get_mut = "pub(crate)")]
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
    ///   placed. Identifer should already be present in the `Directory` table or should be a
    ///   `SystemFolder`.
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
    /// assert_eq!(table, vec!["child_dir1", "child_dir2", "ProgramFiles", "SourceDir"]);
    /// ```
    pub fn add_path<P: Into<PathBuf>, I: Into<Identifier>>(
        &mut self,
        path: P,
        install_path_identifier: I,
    ) -> anyhow::Result<()> {
        let dir = Directory::from_path(path);
        todo!()
    }

    pub fn add_directory<I: Into<Identifier>>(
        mut self,
        parent_id: I,
        subject: Directory,
    ) -> anyhow::Result<Self> {
        let parent_id = parent_id.into();

        // Create the parent directory on-the-fly if one doesn't exist and it is a `SystemFolder`
        // variant. Throw an error otherwise if it doesn't exist.
        if !self.has_directory_id(&parent_id) {
            if let Ok(sf) = SystemFolder::try_from(parent_id.clone()) {
                self = self.add_directory(SystemFolder::TARGETDIR, sf.try_into()?)?;
            } else {
                bail!(WhimsiError::DirectoryNotFound {
                    identifier: parent_id.clone()
                })
            }
        }

        let (id, dao) = match subject {
            Directory::SystemDirectory(ref system_directory) => {
                self.add_system_directory(&system_directory)?
            }
            Directory::SubDirectory(ref subdirectory) => {
                self.add_subdirectory(parent_id, subdirectory)?
            }
        };

        // Add the new directory to the table.
        self.tables.directory_mut().add_directory(dao.clone())?;
        self.identifiers.insert(id, TableEntry::Directory(dao));

        // Add all of the contents to the MSI.
        self.add_directory_contents(subject)?;

        Ok(self)
    }

    fn add_directory_contents(&self, subject: Directory) -> anyhow::Result<()> {
        for item in subject.contents() {
            todo!()
        }

        Ok(())
    }

    fn add_system_directory(
        &mut self,
        system_directory: &SystemDirectory,
    ) -> anyhow::Result<(Identifier, DirectoryDao)> {
        let system_folder = *system_directory.system_folder();

        // Disallow TARGETDIR as the subject directory
        ensure!(
            system_folder != SystemFolder::TARGETDIR,
            WhimsiError::SubRootDirectory
        );

        Ok((system_folder.into(), system_folder.into()))
    }

    fn add_subdirectory<I: Into<Identifier>>(
        &mut self,
        parent_id: I,
        subdirectory: &SubDirectory,
    ) -> anyhow::Result<(Identifier, DirectoryDao)> {
        let parent_id = parent_id.into();

        // Create an identifier if one is not already set.
        let id = self.generate_id();

        // Disallow reusing identifiers in the same MSI.
        ensure!(
            !self.has_identifier(&id),
            WhimsiError::IdentifierConflict { identifier: id }
        );

        // Disallow directories using the same identifier.
        ensure!(
            !self.has_directory_id(&id),
            WhimsiError::DirectoryIdentifierConflict { identifier: id }
        );

        todo!()
    }

    pub fn has_identifier(&self, identifier: &Identifier) -> bool {
        self.identifiers.keys().any(|i| i == identifier)
    }

    pub fn has_directory_id(&self, identifier: &Identifier) -> bool {
        self.tables.directory().has_directory_id(&identifier)
    }

    pub fn has_property(&self, identifier: &Identifier) -> bool {
        self.tables.property().has_property(&identifier)
    }

    /// Generate an `Identifier` not already in the `Identifiers` hashmap.
    /// Identifier is 72 characters to reduce likelihood of collision. 72 Character limit is taken
    /// from Directory table column max char length.
    fn generate_id(&self) -> Identifier {
        loop {
            let mut id_string = "_".to_string();
            Alphanumeric.append_string(&mut rand::rng(), &mut id_string, 71);
            let id = Identifier::from_str(&id_string).unwrap();
            if !self.has_identifier(&id) {
                return id;
            }
        }
    }
}

#[derive(Debug, Error, PartialEq)]
pub enum WhimsiError {
    #[error("Property with identifier {identifier} not found in Property table")]
    InvalidTargetDirChild { identifier: Identifier },
    #[error("TARGETDIR cannot be a subdirectory")]
    SubRootDirectory,
    #[error("Directory with identifier {identifier} not found in Directory table")]
    DirectoryNotFound { identifier: Identifier },
    #[error("Directory with ID {identifier} already exists in Directory Table")]
    DirectoryIdentifierConflict { identifier: Identifier },
    #[error("Identifier {identifier} already exists for MSI. Identifiers must be unique.")]
    IdentifierConflict { identifier: Identifier },
}

#[cfg(test)]
mod test {
    use crate::{
        MsiBuilder, WhimsiError,
        types::{
            column::identifier::Identifier,
            helpers::{
                directory::{Directory, SubDirectory},
                filename::Filename,
            },
            properties::system_folder::SystemFolder,
        },
    };

    #[test]
    fn non_existent_parent_directory() {
        let mut msi = MsiBuilder::default();
        let parent_id = "nonexistent".parse::<Identifier>().unwrap();

        let id: Identifier = "test_id".parse().unwrap();
        let invalid_dir: SubDirectory = "test".parse::<Filename>().unwrap().into();
        let invalid_dir: Directory = invalid_dir.into();

        let expected = WhimsiError::DirectoryNotFound {
            identifier: parent_id.clone(),
        };
        let result = msi.add_directory(parent_id, invalid_dir);
        let actual = result.unwrap_err().downcast().unwrap();
        assert_eq!(expected, actual);
    }
}
