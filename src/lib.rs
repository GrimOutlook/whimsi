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
use crate::tables::file::helper::File;
use crate::tables::media::helper::Media;
use crate::tables::table_entry::TableEntry;
use crate::types::column::identifier::ToIdentifier;
use crate::types::helpers::directory_item::DirectoryItem;
use std::io::{Read, Seek, Write};
use std::{collections::HashMap, path::PathBuf, str::FromStr};

use anyhow::{bail, ensure};
use getset::Getters;
use msi::Package;
use rand::distr::{Alphanumeric, SampleString};
use tables::MsiBuilderTables;
use tables::builder_table::MsiBuilderTable;
use tables::component::dao::ComponentDao;
use tables::component::helper::Component;
use tables::directory::kind::DirectoryKind;
use tables::directory::system_directory::SystemDirectory;
use tables::file::dao::FileDao;
use tables::meta::MetaInformation;
use thiserror::Error;
use types::column::sequence::Sequence;
use types::{
    column::{ColumnValue, identifier::Identifier},
    properties::system_folder::SystemFolder,
};
type Identifiers = Vec<Identifier>;

/// An in-memory representation of the final MSI to be created.
#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct MsiBuilder {
    /// Tracks identifiers used to relate items between tables.
    #[getset(get_mut = "pub(crate)")]
    identifiers: Identifiers,
    tables: MsiBuilderTables,

    /// Information about the whole package. Tracks both information for creating the MSI and
    /// information that is tracked in the _SummaryInformation table.
    meta: MetaInformation,

    /// Contains the directory structure that will be written to the MSI. Includes files and other
    /// components that are contained within directories.
    ///
    /// TODO: Determine if a separate list for `File`s and other things are needed if it's possible
    /// for them to not be contained in a `Directory`.
    system_directories: Vec<SystemDirectory>,
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
    /// # use whimsi_lib::tables::directory::kind::DirectoryKind;
    /// # use whimsi_lib::tables::directory::system_directory::SystemDirectory;
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
    /// # let file_2 = child_dir2.child("file2.pdf");
    /// # file_2.touch().unwrap();
    /// # let temp_dir_path = temp_dir.path();
    ///
    /// // path/to/temp_dir/
    /// // | - file1.txt
    /// // | child_dir1/
    /// // | child_dir2/
    /// //   | - file2.pdf
    /// // With a file system that looks like the above and using ProgramFiles for the
    /// // install_path_identifier
    ///
    /// let mut msi = MsiBuilder::default().with_path(temp_dir_path, SystemFolder::ProgramFilesFolder).unwrap();
    ///
    /// // You will end up with the following on the windows install.
    /// // C:/ProgramFiles/
    /// // | - file1.txt
    /// // | child_dir1/
    /// // | child_dir2/
    /// //   | - file2.pdf
    ///
    /// let sys_dirs = msi.system_directories();
    /// sys_dirs.iter().for_each(SystemDirectory::print_structure);
    /// // 1 entry for the system folder
    /// assert_eq!(sys_dirs.len(), 1, "Number of system directories incorrect");
    /// // 2 directories that were parsed from the path and placed in the system folder that was
    /// // given
    /// let system_directory = sys_dirs.get(0).unwrap();
    /// assert_eq!(system_directory.system_folder(), &SystemFolder::ProgramFilesFolder, "System folder constructed incorrectly");
    /// assert_eq!(system_directory.contents().len(), 3, "Number of system folder contents incorrect");
    /// assert_eq!(system_directory.contained_directories().len(), 2, "Number of directories in system folder incorrect");
    /// assert_eq!(system_directory.contained_files().len(), 1, "Number of files in system folder incorrect");
    /// let child_dir1 = system_directory.contained_directory_by_name("child_dir1").unwrap();
    /// assert_eq!(child_dir1.contents().len(), 0, "child_dir1 contents incorrect");
    /// assert_eq!(child_dir1.contained_directories().len(), 0, "Number of directories in child_dir1 incorrect");
    /// assert_eq!(child_dir1.contained_files().len(), 0, "Number of files in child_dir1 incorrect");
    /// let child_dir2 = system_directory.contained_directory_by_name("child_dir2").unwrap();
    /// assert_eq!(child_dir2.contents().len(), 1, "child_dir2 contents incorrect");
    /// assert_eq!(child_dir2.contained_directories().len(), 0, "Number of directories in child_dir2 incorrect");
    /// assert_eq!(child_dir2.contained_files().len(), 1, "Number of files in child_dir2 incorrect");
    /// ```
    pub fn with_path<P: Into<PathBuf>>(
        mut self,
        path: P,
        parent: SystemFolder,
    ) -> anyhow::Result<Self> {
        let directory = Directory::try_from(path.into())?;
        let mut parent_dir = SystemDirectory::from(parent);
        for item in directory.contents() {
            parent_dir.add_item(item.clone());
        }
        self = self.with_directory(parent_dir)?;
        Ok(self)
    }

    pub fn with_directory(mut self, subject: SystemDirectory) -> anyhow::Result<Self> {
        if self
            .system_directories()
            .iter()
            .find(|other| subject.name_conflict(other))
            .is_some()
        {
            todo!("Create error for when the system directory already exists")
        }

        self.system_directories.push(subject);

        Ok(self)
    }

    /// Build the MSI from all information given to MSIBuilder.
    pub fn build<F: Read + Write + Seek>(self, container: F) -> anyhow::Result<Package<F>> {
        let mut package = Package::create(*self.meta.package_type(), container)?;
        self.tables.write_to_package(&mut package);
        Ok(package)
    }

    pub fn get_system_directory(&self, folder: SystemFolder) -> Option<&SystemDirectory> {
        self.system_directories
            .iter()
            .find(|dir| *dir.system_folder() == folder)
    }

    pub fn has_identifier(&self, identifier: &Identifier) -> bool {
        self.identifiers.iter().any(|i| i == identifier)
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

impl Default for MsiBuilder {
    fn default() -> Self {
        Self {
            identifiers: vec![Identifier::from(SystemFolder::TARGETDIR)],
            tables: Default::default(),
            system_directories: Vec::new(),
            meta: MetaInformation::default(),
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
