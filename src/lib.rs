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

use crate::tables::directory::dao::DirectoryDao;
use crate::tables::directory::helper::Directory;
use crate::tables::directory::helper::DirectoryKind;
use crate::tables::directory::helper::SubDirectory;
use crate::tables::directory::helper::SystemDirectory;
use crate::tables::file::helper::File;
use crate::tables::media::helper::Media;
use crate::tables::table_entry::TableEntry;
use crate::types::column::identifier::ToIdentifier;
use crate::types::helpers::directory_item::DirectoryItem;
use std::{collections::HashMap, path::PathBuf, process::id, str::FromStr};

use anyhow::{bail, ensure};
use getset::Getters;
use rand::distr::{Alphanumeric, SampleString};
use tables::MsiBuilderTables;
use tables::builder_table::MsiBuilderTable;
use tables::component::dao::ComponentDao;
use tables::component::helper::Component;
use tables::file::dao::FileDao;
use thiserror::Error;
use types::column::sequence::Sequence;
use types::{
    column::{ColumnValue, identifier::Identifier},
    properties::system_folder::SystemFolder,
};
type Identifiers = Vec<Identifier>;

/// An in-memory representation of the final MSI to be created.
#[derive(Debug, Default, Getters)]
#[getset(get = "pub")]
pub struct MsiBuilder {
    /// Tracks identifiers used to relate items between tables.
    #[getset(get_mut = "pub(crate)")]
    identifiers: Identifiers,
    tables: MsiBuilderTables,

    /// Contains the directory structure that will be written to the MSI. Includes files and other
    /// components that are contained within directories.
    ///
    /// TODO: Determine if a separate list for `File`s and other things are needed if it's possible
    /// for them to not be contained in a `Directory`.
    directories: Vec<Directory>,
}

impl MsiBuilder {
    /// Insert a given filesystem path's contents into the MSI for installation.
    ///
    /// If the path leads to a directory, the directory and all contents will be recursively added
    /// to the MSI.
    ///
    /// If the path leads to a file, only the file will be added to the MSI.
    ///
    /// ## Arguments
    ///
    /// - *path* Path to the items you want to be copied to the system on install.
    /// - *install_path_identifier* `Identifier` for the directory where the given path should be
    ///   placed. Identifer should already be present in the `Directory` table or should be a
    ///   `SystemFolder`. Most commonly you will want to use `SystemFolder::VARIANT` for this
    ///   parameter.
    ///
    /// ## Example
    ///
    /// ```
    /// # use whimsi_lib::MsiBuilder;
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
    /// msi = msi.add_path(temp_dir.path(), SystemFolder::ProgramFiles).unwrap();
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
    pub fn add_path<P: Into<PathBuf>, I: Into<Identifier>>(
        mut self,
        path: P,
        install_path_identifier: I,
    ) -> anyhow::Result<Self> {
        Ok(self.add_directory_item(
            DirectoryItem::try_from(path.into())?,
            &install_path_identifier.into(),
        )?)
    }

    /// Add a given directory item to the Msi for installation
    fn add_directory_item(
        mut self,
        item: DirectoryItem,
        parent_id: &Identifier,
    ) -> anyhow::Result<Self> {
        match item {
            DirectoryItem::Directory(directory) => {
                self = self.add_directory(parent_id.clone(), directory)?
            }
            DirectoryItem::File(file) => self = self.add_file(file, parent_id)?,
        };

        Ok(self)
    }

    fn add_directory<I: Into<Identifier>>(
        mut self,
        parent_id: I,
        subject: Directory,
    ) -> anyhow::Result<Self> {
        let parent_id = parent_id.into();

        // Create the parent directory on-the-fly if one doesn't exist and it is a `SystemFolder`
        // variant. Throw an error otherwise if it doesn't exist.
        let directory = match self.directory_mut_by_id(&parent_id) {
            Some(directory) => directory,
            None => {
                if let Ok(sf) = SystemFolder::try_from(parent_id.clone()) {
                    self = self.add_directory(SystemFolder::TARGETDIR, sf.try_into()?)?;
                    self.directory_mut_by_id(&parent_id).unwrap()
                } else {
                    bail!(WhimsiError::DirectoryNotFound {
                        identifier: parent_id.clone()
                    })
                }
            }
        };

        directory.add(subject);

        Ok(self)
    }

    pub fn add_file(&self, file: File) -> anyhow::Result<Self> {
        todo!()
    }

    pub fn has_identifier(&self, identifier: &Identifier) -> bool {
        self.identifiers.iter().any(|i| i == identifier)
    }

    pub fn directory_mut_by_id(&self, identifer: &Identifier) -> Option<&Directory> {
        self.directories.iter().find(|dir| {
            if let Some(system_dir) = dir.try_as_system_directory_ref() {
                *identifer == system_dir.system_folder().into()
            } else {
                false
            }
        })
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
        tables::directory::helper::{Directory, SubDirectory},
        types::{
            column::identifier::Identifier, helpers::filename::Filename,
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
        let result = msi.add_directory_to_tables(parent_id, invalid_dir);
        let actual = result.unwrap_err().downcast().unwrap();
        assert_eq!(expected, actual);
    }
}
