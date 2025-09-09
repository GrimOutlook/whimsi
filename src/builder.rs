use std::cell::RefCell;
use std::io::Read;
use std::io::Seek;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::rc::Rc;
use std::str::FromStr;

use anyhow::Context;
use anyhow::bail;
use getset::Getters;
use itertools::Itertools;
use once_cell::sync::OnceCell;
use once_cell::unsync::Lazy;
use rand::distr::Alphanumeric;
use rand::distr::SampleString;
use tracing::debug;
use tracing::info;

use crate::constants::*;
use crate::tables::builder_list::MsiBuilderList;
use crate::tables::builder_table::MsiBuilderTable;
use crate::tables::component::dao::ComponentDao;
use crate::tables::component::table::ComponentIdentifier;
use crate::tables::component::table::ComponentTable;
use crate::tables::dao::Dao;
use crate::tables::directory::dao::DirectoryDao;
use crate::tables::directory::directory_identifier::DirectoryIdentifier;
use crate::tables::directory::table::DirectoryTable;
use crate::tables::feature::identifier::FeatureIdentifier;
use crate::tables::feature::table::FeatureTable;
use crate::tables::feature_components::dao::FeatureComponentsDao;
use crate::tables::feature_components::table::FeatureComponentsTable;
use crate::tables::file::dao::FileDao;
use crate::tables::file::table::FileIdentifier;
use crate::tables::file::table::FileTable;
use crate::tables::id_generator_builder_list::IdGeneratorBuilderList;
use crate::tables::media::cabinet_identifier::CabinetHandle;
use crate::tables::media::cabinet_identifier::CabinetIdentifier;
use crate::tables::media::dao::MediaDao;
use crate::tables::media::table::MediaTable;
use crate::tables::meta::MetaInformation;
use crate::tables::msi_file_hash::dao::MsiFileHashDao;
use crate::tables::msi_file_hash::table::MsiFileHashTable;
use crate::tables::property::table::PropertyTable;
use crate::tables::registry::table::RegistryTable;
use crate::types::column::default_dir::DefaultDir;
use crate::types::column::filename::Filename;
use crate::types::column::identifier::Identifier;
use crate::types::column::identifier::ToIdentifier;
use crate::types::column::sequence::Sequence;
use crate::types::helpers::cabinet_info::CabinetInfo;
use crate::types::helpers::cabinets::Cabinets;
use crate::types::helpers::id_generator::IdGenerator;
use crate::types::properties::system_folder::SystemFolder;

/// An in-memory representation of the final MSI to be created.
#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct MsiBuilder {
    /// Information about the whole package. Tracks both information for
    /// creating the MSI and information that is tracked in the
    /// _SummaryInformation table.
    meta: MetaInformation,

    /// A list of all identifiers used in this MSI. Used to ensure no duplicate
    /// Identifiers are created.
    identifiers: Rc<RefCell<Vec<Identifier>>>,

    cabinets: Cabinets,
    component: ComponentTable,
    directory: DirectoryTable,
    file: FileTable,
    media: MediaTable,
    feature: FeatureTable,
    feature_components: FeatureComponentsTable,
    // TODO: Ensure that the following properties are defined:
    // - ProductCode
    // - ProductName
    // - ProductVersion
    // - ProductLanguage
    // - Manufacturer
    // - UpgradeCode
    // - ALLUSERS
    property: PropertyTable,
    registry: RegistryTable,
    msi_file_hash: MsiFileHashTable,
    // admin_execute_sequence: AdminExecuteSequenceTable,
    // admin_ui_sequence: AdminUiSequenceTable,
    // advt_execute_sequence: AdvtExecuteSequenceTable,
    // install_execute_sequence: InstallExecuteSequenceTable,
    // install_ui_sequence: InstallUiSequenceTable,
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
        let parent = parent.into();
        debug!("Adding path [{:?}] contents to directory [{}]", path, parent);

        let directory_contents: Vec<std::fs::DirEntry> =
            std::fs::read_dir(&path)?.try_collect()?;
        for item in directory_contents {
            let filetype = item.file_type().expect(&format!(
                "Failed to get file type for file {:?}",
                item
            ));
            let path = item.path();

            if filetype.is_file() {
                self = self.with_file_path(path, parent.clone())?;
            } else if filetype.is_dir() {
                let id = self.add_directory_from_path(&path, parent.clone())?;
                self = self.with_path_contents(path, id)?;
            } else {
                bail!("Create error for nonfile+nondir types")
            }
        }

        Ok(self)
    }

    pub fn add_directory_from_path(
        &mut self,
        path: impl Into<PathBuf>,
        parent: impl Into<DirectoryIdentifier>,
    ) -> anyhow::Result<DirectoryIdentifier> {
        let path = path.into();

        let directory_id = self.directory.generate_id();
        let name = path
            .file_name()
            .with_context(|| format!(
                "Directory path [{path:?}] ended with `..` which is illegal."
            ))?
            .to_str()
            .with_context(|| format!("Directory path [{path:?}] has invlaid unicode"))?;
        self.add_directory(name, parent)
    }

    pub fn add_directory(
        &mut self,
        name: impl ToString,
        parent: impl Into<DirectoryIdentifier>,
    ) -> anyhow::Result<DirectoryIdentifier> {
        let filename = Filename::from_str(&name.to_string())?;
        let id = self.directory.generate_id();
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
        IdGeneratorBuilderList::add(&mut self.directory, dao)
    }

    pub fn with_file_path(
        mut self,
        path: impl Into<PathBuf>,
        parent_id: impl Into<DirectoryIdentifier>,
    ) -> anyhow::Result<Self> {
        let path = path.into();
        debug!("Creating DAOs for {path:?}");

        let file_id = self.file.generate_id();
        let component_id = self.component.generate_id();
        let file_hash_dao = MsiFileHashDao::from_path(file_id.clone(), &path)?;
        self.add_to_tables(file_hash_dao)?;
        self.add_to_default_feature(&component_id)?;
        let sequence = self.add_to_media(file_id.clone(), path.clone());
        let file_dao = FileDao::install_file_from_path(
            file_id.clone(),
            component_id.clone(),
            path,
            sequence,
        )?;
        self.add_to_tables(file_dao)?;
        let component_dao = ComponentDao::new(component_id, parent_id.into())
            .with_keypath(file_id.to_identifier());
        self.add_to_tables(component_dao)?;
        Ok(self)
    }

    /// Adds the given file to media so it will be installed when the MSI is
    /// run.
    ///
    /// If no media entry exists yet, one is created along with a cabinet file.
    fn add_to_media(
        &mut self,
        file_id: FileIdentifier,
        file_path: PathBuf,
    ) -> Sequence {
        debug!("Adding file [{file_id}] with path [{file_path:?}] to media");
        // Verify there is a Media entry to add on to.
        if self.media.is_empty() {
            // Create a new cabinet file.
            let cabinet_id = self.new_cabinet();
            // Create a new media DAO.
            let dao = MediaDao::internal(1, cabinet_id.clone())
                .expect("Creating first entry to Media table failed");
            self.add_to_tables(dao);
        }

        let media_dao = self
            .media
            .get_last_internal_media_mut()
            .expect("Media table contains no internal CAB files");

        // Add the file to the cabinet.
        let cabinet_id = &media_dao
            .cabinet_id()
            .expect("Media DAO that had a cabinet ID apparently doesn't");
        let mut cabinet_info =
            self.cabinets.find_id_mut(cabinet_id).expect(&format!("Cabinet of ID [{}] referenced by media with disk ID [{}] was not found", cabinet_id, media_dao.disk_id()));
        cabinet_info.add_file(file_id, file_path);
        // Set the Media table entry's LastSequence to the number of files in
        // the cabinet.
        media_dao
            .set_last_sequence(cabinet_info)
            .expect("LastSequence got too large. TODO: Handle this case.")
    }

    /// Creates a new cabinet file and returns the ID.
    fn new_cabinet(&mut self) -> CabinetHandle {
        let id = self.cabinets.generate_id();
        self.cabinets.add_new(id.clone());
        CabinetHandle::Internal(id)
    }

    /// Build the MSI from all information given to MSIBuilder.
    pub fn build<F: std::io::Read + std::io::Write + std::io::Seek>(
        self,
        container: F,
    ) -> anyhow::Result<msi::Package<F>> {
        info!("Building MSI");
        let mut package =
            msi::Package::create(*self.meta.package_type(), container)?;
        self.write_tables_to_package(&mut package)?;
        self.write_cabinets_to_package(&mut package)?;
        info!("Finished building MSI");
        Ok(package)
    }

    /// Just writes the information stored in each of the table properties to
    /// the package tables.
    ///
    /// Information is written based on a predetermined order so that
    /// information that doesn't reference other table information is
    /// written first.
    pub(crate) fn write_tables_to_package<
        F: std::io::Read + std::io::Write + std::io::Seek,
    >(
        &self,
        package: &mut msi::Package<F>,
    ) -> anyhow::Result<()> {
        info!("Writing tables to package");
        self.directory.write_to_package(package)?;
        self.component.write_to_package(package)?;
        self.file.write_to_package(package)?;
        self.media.write_to_package(package)?;
        self.feature.write_to_package(package)?;
        self.feature_components.write_to_package(package)?;
        self.property.write_to_package(package)?;
        self.registry.write_to_package(package)?;
        self.msi_file_hash.write_to_package(package)?;
        Ok(())
    }

    pub(crate) fn write_cabinets_to_package<
        F: std::io::Read + std::io::Write + std::io::Seek,
    >(
        &self,
        package: &mut msi::Package<F>,
    ) -> anyhow::Result<()> {
        let previous_last_sequence = 1;
        for media in MsiBuilderTable::entries(&self.media)
            .iter()
            .sorted_by_key(|dao| Into::<i16>::into(*dao.last_sequence()))
        {
            let Some(cabinet_id) = media.cabinet_id() else {
                // Ignore media listings that don't represent an internal cabinet file
                continue;
            };
            let last_sequence = Into::<i16>::into(*media.last_sequence());
            if last_sequence == 0 {
                // Skip this cabinet file if no files are to be written to it.
                continue;
            }
            let cabinet_info = self.cabinets.find_id(&cabinet_id).expect(
                "Cabinet of ID [{}] could not be found when trying to build it!",
            );
            let files = self
                .file
                .in_sequence_range(previous_last_sequence, last_sequence);
            if files.len() == 0 {
                unreachable!(
                    "No files found for given cabinet file. This should not happen."
                )
            }

            let mut cabinet_file = self.create_cabinet_file(&cabinet_info)?;
            // Have to set the poisition of the file reader back to 0 so that it gets read from the
            // beginning when it gets read again.
            cabinet_file.rewind();

            self.write_cabinet_to_package(
                &cabinet_info,
                &mut cabinet_file,
                package,
            )?;
        }
        Ok(())
    }

    pub(crate) fn create_cabinet_file(
        &self,
        cabinet_info: &CabinetInfo,
    ) -> anyhow::Result<std::fs::File> {
        debug!("Creating cabinet file [{}]", cabinet_info.id());
        let mut cab_builder = cab::CabinetBuilder::new();
        let mut folder = cab_builder.add_folder(cab::CompressionType::MsZip);
        cabinet_info.files().iter().for_each(|file| {
            /// NOTE: From what I can tell attributes only need to be set on files in the File
            /// table as those attributes overwrite the attributes that are set in the cabinet
            /// file.
            folder.add_file(file.id().to_string());
        });
        let file = tempfile::tempfile().with_context(|| {
            format!(
                "Failed to create tempfile for cabinet [{}]",
                cabinet_info.id()
            )
        })?;
        let mut cab_writer = cab_builder.build(file).with_context(|| {
            format!(
                "Failed to create cabinet file writer for cabinet file [{}]",
                cabinet_info.id()
            )
        })?;

        let mut files_iter = cabinet_info.files().iter();
        while let Some(mut writer) =
            cab_writer.next_file().expect("Failed to open")
            && let Some(file) = files_iter.next()
        {
            let mut reader = std::fs::File::open(file.path()).unwrap();
            std::io::copy(&mut reader, &mut writer).unwrap();
        }

        Ok(cab_writer.finish()?)
    }

    pub(crate) fn write_cabinet_to_package<
        F: std::io::Read + std::io::Write + std::io::Seek,
    >(
        &self,
        cabinet_info: &CabinetInfo,
        cabinet: &mut std::fs::File,
        package: &mut msi::Package<F>,
    ) -> anyhow::Result<()> {
        let cabinet_id = cabinet_info.id();
        debug!(
            "Writing cabinet file for cabinet ID [{}] to package",
            cabinet_id
        );
        let mut writer = package
            .write_stream(&cabinet_id.to_string())
            .context("Failed to create stream writier for package")?;
        std::io::copy(cabinet, &mut writer).with_context(|| {
            format!("Failed to copy cabinet [{cabinet_id}] to package")
        })?;

        Ok(())
    }

    fn add_to_default_feature(
        &mut self,
        component_id: &ComponentIdentifier,
    ) -> anyhow::Result<()> {
        // Get the default feature DAO.
        let Some(default) = self.feature.get_default_feature() else {
            bail!("No default feature could be found");
        };
        // Add the component to the default feature.
        self.add_component_to_feature(&default.feature().clone(), component_id)
    }

    fn add_component_to_feature(
        &mut self,
        feature_id: &FeatureIdentifier,
        component_id: &ComponentIdentifier,
    ) -> anyhow::Result<()> {
        self.feature_components.add(FeatureComponentsDao::new(
            feature_id.clone(),
            component_id.clone(),
        ))
    }

    /// Insert the given DAO into it's respective table.
    pub fn add_to_tables(&mut self, dao: impl Into<Dao>) -> anyhow::Result<()> {
        let dao = Into::<Dao>::into(dao);
        match dao {
            Dao::Component(component_dao) => {
                IdGeneratorBuilderList::add(&mut self.component, component_dao)
            }
            Dao::Directory(directory_dao) => {
                IdGeneratorBuilderList::add(&mut self.directory, directory_dao)
            }
            Dao::File(file_dao) => {
                IdGeneratorBuilderList::add(&mut self.file, file_dao)
            }
            Dao::Registry(registry_dao) => {
                IdGeneratorBuilderList::add(&mut self.registry, registry_dao)
            }
            Dao::Feature(feature_dao) => {
                IdGeneratorBuilderList::add(&mut self.feature, feature_dao)
            }
            Dao::Property(dao) => self.property.add(dao),
            Dao::Media(dao) => self.media.add(dao),
            Dao::MsiFileHash(dao) => self.msi_file_hash.add(dao),
            Dao::FeatureComponents(dao) => self.feature_components.add(dao),
        }
    }
}

impl Default for MsiBuilder {
    fn default() -> Self {
        let empty_entries = Rc::new(RefCell::new(Vec::new()));
        Self {
            meta: Default::default(),

            // Tables that don't have IDs for their entries.
            property: Default::default(),
            media: Default::default(),
            feature_components: Default::default(),
            msi_file_hash: Default::default(),

            // Non-tables that need access to all or generate entity IDs.
            identifiers: empty_entries.clone(),
            cabinets: Cabinets::new(empty_entries.clone()),

            // Tables that can generate IDs for their entries.
            component: ComponentTable::new(empty_entries.clone()),
            directory: DirectoryTable::new(empty_entries.clone()),
            feature: FeatureTable::new(empty_entries.clone()),
            file: FileTable::new(empty_entries.clone()),
            registry: RegistryTable::new(empty_entries.clone()),
        }
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
