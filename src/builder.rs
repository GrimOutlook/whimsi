use std::os::unix::process::parent_id;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::bail;
use getset::Getters;
use itertools::Itertools;
use rand::distr::Alphanumeric;
use rand::distr::SampleString;
use tracing::debug;
use tracing::info;

use crate::constants::*;
use crate::tables::builder_table::MsiBuilderTable;
use crate::tables::component::table::ComponentTable;
use crate::tables::directory::dao::DirectoryDao;
use crate::tables::directory::directory_identifier::DirectoryIdentifier;
use crate::tables::directory::table::DirectoryTable;
use crate::tables::file::dao::FileDao;
use crate::tables::file::table::FileTable;
use crate::tables::media::table::MediaTable;
use crate::tables::meta::MetaInformation;
use crate::tables::property::table::PropertyTable;
use crate::types::column::default_dir::DefaultDir;
use crate::types::column::identifier::Identifier;
use crate::types::helpers::filename::Filename;
use crate::types::properties::system_folder::SystemFolder;

/// An in-memory representation of the final MSI to be created.
#[derive(Debug, Default, Getters)]
#[getset(get = "pub")]
pub struct MsiBuilder {
    /// Information about the whole package. Tracks both information for
    /// creating the MSI and information that is tracked in the
    /// _SummaryInformation table.
    meta: MetaInformation,

    /// A list of all identifiers used in this MSI. Used to ensure no duplicate
    /// Identifiers are created.
    identifiers: Vec<Identifier>,

    component: ComponentTable,
    directory: DirectoryTable,
    file: FileTable,
    media: MediaTable,
    property: PropertyTable,
}

impl MsiBuilder {
    /// Insert a given filesystem path's contents into the MSI for installation.
    ///
    /// If the path leads to a directory, the directory and all contents will be
    /// recursively added to the MSI.
    ///
    /// If the path leads to a file, only the file will be added to the MSI.
    ///
    /// ## Arguments
    ///
    /// - *path* Path to the directory you want to be copied to the system on
    ///   install.
    /// - *parent* `Identifier` for the directory where the given path should be
    ///   placed. Identifer should already be present in the `Directory` table
    ///   or should be a `SystemFolder`. Most commonly you will want to use
    ///   `SystemFolder::VARIANT` for this parameter.
    ///
    /// ## Returns
    /// An updated instance of the MsiBuilder object, with the contents of the
    /// path stored in the database or Err() if an error was encountered.
    ///
    /// ## Example
    ///
    /// ```
    /// # use whimsi_lib::builder::MsiBuilder;
    /// # use whimsi_lib::tables::directory::container::Container;
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
    pub fn with_path_contents(
        mut self,
        path: impl Into<PathBuf>,
        parent: impl Into<DirectoryIdentifier>,
    ) -> anyhow::Result<Self> {
        let path = path.into();

        let directory_id = self.generate_id();
        let directory_name = path
            .file_name()
            .ok_or(MsiBuilderError::NoDirectoryName { path: path.clone() })?
            .to_str()
            .ok_or(MsiBuilderError::InvalidDirectoryName {
                path: path.clone(),
            })?;
        let id = self.add_directory(directory_name, parent)?;

        let directory_contents: Vec<std::fs::DirEntry> =
            std::fs::read_dir(&path)?.try_collect()?;
        for item in directory_contents {
            let filetype = item.file_type().expect(&format!(
                "Failed to get file type for file {:?}",
                item
            ));
            let path = item.path();

            if filetype.is_file() {
                self = self.with_file_path(path, id.clone())?;
            } else if filetype.is_dir() {
                self = self.with_path_contents(path, id.clone())?;
            } else {
                bail!("Create error for nonfile+nondir types")
            }
        }

        Ok(self)
    }

    pub fn add_directory(
        &mut self,
        name: impl ToString,
        parent: impl Into<DirectoryIdentifier>,
    ) -> anyhow::Result<Identifier> {
        let filename = Filename::from_str(&name.to_string())?;
        let id = self.generate_id();
        self.add_directory_dao(DirectoryDao::new(
            filename,
            id.clone(),
            parent,
        ))?;
        Ok(id)
    }

    pub fn add_directory_dao(
        &mut self,
        dao: DirectoryDao,
    ) -> anyhow::Result<()> {
        self.directory.add(dao)
    }

    pub fn with_file_path(
        mut self,
        path: impl Into<PathBuf>,
        parent_id: impl Into<DirectoryIdentifier>,
    ) -> anyhow::Result<Self> {
        todo!()
        // let dao = FileDao::new(file, component, name, size, version, language, attributes, sequence)
    }

    /// Build the MSI from all information given to MSIBuilder.
    pub fn build<F: std::io::Read + std::io::Write + std::io::Seek>(
        self,
        container: F,
    ) -> anyhow::Result<msi::Package<F>> {
        info!("Building MSI");
        let mut package =
            msi::Package::create(*self.meta.package_type(), container)?;
        self.write_to_package(&mut package)?;
        Ok(package)
    }

    /// Just writes the information stored in each of the table properties to
    /// the package tables.
    ///
    /// Information is written based on a predetermined order so that
    /// information that doesn't reference other table information is
    /// written first.
    pub(crate) fn write_to_package<
        F: std::io::Read + std::io::Write + std::io::Seek,
    >(
        &self,
        package: &mut msi::Package<F>,
    ) -> anyhow::Result<()> {
        info!("Writing tables to package");
        self.directory.write_to_package(package)?;
        self.component.write_to_package(package)?;
        self.file.write_to_package(package)?;
        // self.media.write_to_package(package);
        // self.property.write_to_package(package);
        Ok(())
    }

    /// Generate an `Identifier` not already listed in the `Identifiers` list.
    /// Returns the generated `Identifier` to the user for use and adds a
    /// clone to the `Identifiers` list so it cannot be generated and used
    /// again.
    ///
    /// # Panics
    /// If a randomly generated ID is not a valid `Identifier`. Should not be
    /// possible given the random string characterset and parsing rules of
    /// an `Identifier`.
    pub fn generate_id(&mut self) -> Identifier {
        loop {
            // Start the generated ID string with an underscore since
            // identifiers aren't allowed to start with a number and
            // this is the simplest way around rerolling if the randomly
            // generated identifier starts with a number.
            let mut id_string = "_".to_string();
            let id_string_len = id_string.len();
            Alphanumeric.append_string(
                &mut rand::rng(),
                &mut id_string,
                IDENTIFIER_MAX_LEN - id_string_len,
            );
            let id = Identifier::from_str(&id_string).unwrap();
            if !self.has_identifier(&id) {
                // Ensure that the identifier cannot be generated again.
                self.identifiers.push(id.clone());
                return id;
            }
        }
    }

    pub fn has_identifier(&self, identifier: &Identifier) -> bool {
        self.identifiers.iter().any(|i| i == identifier)
    }
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum MsiBuilderError {
    #[error(
        "Property with identifier {identifier} not found in Property table"
    )]
    InvalidTargetDirChild { identifier: Identifier },
    #[error("TARGETDIR cannot be a subdirectory")]
    SubRootDirectory,
    #[error(
        "Directory with identifier {identifier} not found in Directory table"
    )]
    DirectoryNotFound { identifier: Identifier },
    #[error("Directory with ID {identifier} already exists in Directory Table")]
    DirectoryIdentifierConflict { identifier: Identifier },
    #[error(
        "Identifier {identifier} already exists for MSI. Identifiers must be unique."
    )]
    IdentifierConflict { identifier: Identifier },
    #[error("No directory name could be found for path [{path}]")]
    NoDirectoryName { path: PathBuf },
    #[error("Invalid directory name found for path [{path}]")]
    InvalidDirectoryName { path: PathBuf },
}
