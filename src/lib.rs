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

use std::{collections::HashMap, path::PathBuf, str::FromStr};

use anyhow::{bail, ensure};
use getset::Getters;
use rand::distr::{Alphanumeric, SampleString};
use tables::MsiBuilderTables;
use thiserror::Error;
use types::{
    column::{ColumnValue, identifier::Identifier},
    dao::directory::DirectoryDao,
    helpers::directory::{Directory, DirectoryKind},
    properties::system_folder::SystemFolder,
};
type Identifiers = HashMap<Identifier, ColumnValue>;

/// An in-memory representation of the final MSI to be created.
#[derive(Default, Getters)]
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
        let dir = Directory::from_path(path, install_path_identifier);
        todo!()
    }

    pub fn add_directory<I: Into<Identifier>>(
        &self,
        parent_id: I,
        subject: Directory,
    ) -> anyhow::Result<()> {
        let parent_id = parent_id.into();

        if let Some(id) = subject.identifier() {
            // Disallow TARGETDIR as the subject directory
            if let Ok(subject_sf) = SystemFolder::try_from(id.clone())
                && subject_sf == SystemFolder::TARGETDIR
            {
                bail!(WhimsiError::SubRootDirectory)
            }

            // If the `parent` is TARGETDIR, we need to verify that `subject` is a valid directory to be
            // placed in it. Valid directories are directories that match Identifiers already defined
            // in the `Property` table or with names that match a `SystemFolder` variant.
            // WARN: We currently don't allow non-system folder Identifiers to be placed in TARGETDIR.
            // TODO: Implement the above. Values defined in the `Property` table are valid for this as
            // well.
            if let Ok(parent_sf) = SystemFolder::try_from(parent_id)
                && parent_sf == SystemFolder::TARGETDIR
            {
                ensure!(
                    self.tables.property().has_property(&id),
                    WhimsiError::PropertyNotFound { identifier: id }
                );
                todo!("Implement non system-folder identifiers in TARGETDIR")
            }
        }

        todo!()
    }

    pub fn has_identifier(&self, identifier: &Identifier) -> bool {
        self.identifiers.keys().any(|i| i == identifier)
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

#[derive(Debug, Error)]
pub enum WhimsiError {
    #[error("Property with identifier {identifier} not found in Property table")]
    PropertyNotFound { identifier: Identifier },
    #[error("TARGETDIR cannot be a subdirectory")]
    SubRootDirectory,
}

#[cfg(test)]
mod test {
    use crate::{
        MsiBuilder, WhimsiError,
        types::{
            column::identifier::Identifier,
            helpers::directory::{Directory, SubDirectory},
            properties::system_folder::SystemFolder,
        },
    };

    #[test]
    fn undefined_properties_not_allowed_in_TARGETDIR() {
        let mut msi = MsiBuilder::default();
        let root_dir = SystemFolder::TARGETDIR;

        let id: Identifier = "test_id".parse().unwrap();
        let invalid_dir = SubDirectory::new("test".parse().unwrap(), id.clone()).into();

        let expected = WhimsiError::PropertyNotFound { identifier: id };
        let actual = msi.add_directory(root_dir, invalid_dir);
    }
}
