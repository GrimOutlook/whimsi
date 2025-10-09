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
use getset::Setters;
use getset::WithSetters;
use itertools::Itertools;
use msi::Insert;
use msi::Value;
use once_cell::sync::OnceCell;
use once_cell::unsync::Lazy;
use rand::distr::Alphanumeric;
use rand::distr::SampleString;
use tracing::debug;
use tracing::info;
use uuid::Uuid;

use crate::constants::*;
use crate::tables::AdminExecuteSequenceTable;
use crate::tables::AdminUiSequenceTable;
use crate::tables::AdvtExecuteSequenceTable;
use crate::tables::AppSearchTable;
use crate::tables::BinaryTable;
use crate::tables::ComponentDao;
use crate::tables::ComponentIdentifier;
use crate::tables::ComponentTable;
use crate::tables::CustomActionTable;
use crate::tables::DirectoryDao;
use crate::tables::DirectoryIdentifier;
use crate::tables::DirectoryTable;
use crate::tables::FeatureComponentsDao;
use crate::tables::FeatureComponentsTable;
use crate::tables::FeatureIdentifier;
use crate::tables::FeatureTable;
use crate::tables::FileDao;
use crate::tables::FileIdentifier;
use crate::tables::FileTable;
use crate::tables::IconTable;
use crate::tables::InstallExecuteSequenceTable;
use crate::tables::InstallUiSequenceTable;
use crate::tables::LaunchConditionTable;
use crate::tables::LockPermissionsTable;
use crate::tables::MediaDao;
use crate::tables::MediaTable;
use crate::tables::MsiFileHashDao;
use crate::tables::MsiFileHashTable;
use crate::tables::MsiTable;
use crate::tables::MsiTableDao;
use crate::tables::PropertyDao;
use crate::tables::PropertyTable;
use crate::tables::RegLocatorTable;
use crate::tables::RegistryTable;
use crate::tables::ServiceControlTable;
use crate::tables::ServiceInstallTable;
use crate::tables::ShortcutTable;
use crate::tables::SignatureTable;
use crate::tables::builder_table::MsiTableKind;
use crate::tables::dao::MsiDao;
use crate::tables::identifier_generator_table::IdentifierGeneratorTable;
use crate::tables::meta::MetaInformation;
use crate::types::column::default_dir::DefaultDir;
use crate::types::column::filename::Filename;
use crate::types::column::identifier::Identifier;
use crate::types::column::identifier::ToIdentifier;
use crate::types::column::sequence::Sequence;
use crate::types::helpers::architecture::MsiArchitecture;
use crate::types::helpers::cabinet_info::CabinetInfo;
use crate::types::helpers::cabinets::CabinetHandle;
use crate::types::helpers::cabinets::Cabinets;
use crate::types::helpers::id_generator::IdentifierGenerator;
use crate::types::helpers::page_count::PageCount;
use crate::types::helpers::security_flag::DocSecurity;
use crate::types::properties::system_folder::SystemFolder;
use crate::types::standard_action::StandardAction;

/// An in-memory representation of the final MSI to be created.
#[derive(Getters, Setters)]
#[getset(get = "pub")]
pub struct MsiBuilder {
    /// Information about the whole package. Tracks both information for
    /// creating the MSI and information that is tracked in the
    /// _SummaryInformation table.
    ///
    /// WARN: The MSI cannot be built without this information being filled out.
    #[getset(set = "pub")]
    meta: Option<MetaInformation>,

    /// A list of all identifiers used in this MSI. Used to ensure no duplicate
    /// Identifiers are created.
    identifiers: Rc<RefCell<Vec<Identifier>>>,

    /// Keeps track of cabinets contained in this MSI.
    cabinets: Cabinets,

    /// List of all the tables managed by this builder
    tables: Vec<MsiTable>,
}

impl MsiBuilder {
    pub fn with_meta(mut self, meta: MetaInformation) -> Self {
        self.meta = Some(meta);
        self
    }

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
            id.clone(),
            Some(parent.into()),
            filename,
        ))?;

        Ok(id)
    }

    pub fn add_directory_dao(
        &mut self,
        dao: DirectoryDao,
    ) -> anyhow::Result<()> {
        if let Some(parent) = dao.parent_directory()
            && let Ok(system_folder) =
                SystemFolder::try_from(parent.to_identifier())
        {
            // Just ignore any errors when adding this directory since this will likely already be
            // in the table.
            let _ = self.add_directory_dao(system_folder.into());
        }
        IdentifierGeneratorTable::add(&mut self.directory, dao)
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

    pub fn add_property(
        &mut self,
        key: impl ToString,
        value: impl ToString,
    ) -> anyhow::Result<()> {
        self.property.add(PropertyDao::new(
            key.to_string().parse()?,
            value.to_string().parse()?,
        ));
        Ok(())
    }

    pub fn with_property(
        mut self,
        key: impl ToString,
        value: impl ToString,
    ) -> anyhow::Result<Self> {
        self.add_property(key, value)?;
        Ok(self)
    }

    /// Build the MSI from all information given to MSIBuilder.
    pub fn build<F: std::io::Read + std::io::Write + std::io::Seek>(
        self,
        mut container: F,
    ) -> anyhow::Result<msi::Package<F>> {
        let Some(ref meta) = self.meta else {
            bail!("Meta information cannot be blank");
        };
        info!("Building MSI");

        // Copy the information from the blank reference MSI to the container.
        let mut reference_msi =
            std::io::Cursor::new(include_bytes!("../resources/Schema.msi"));
        std::io::copy(&mut reference_msi, &mut container);
        let mut package = msi::Package::open(container)?;

        // TODO: Remove after getting everything working
        let extra_tables = [
            "ActionText",
            // "Directory",
            // "Media",
            // "File",
            // "Shortcut",
            // "CustomAction",
            "AdvtUISequence",
            "AppId",
            "BBControl",
            "Billboard",
            "BindImage",
            "CCPSearch",
            "CheckBox",
            "Class",
            "ComboBox",
            "CompLocator",
            "Complus",
            // "Control",
            "ControlCondition",
            "ControlEvent",
            "Dialog",
            "DrLocator",
            "DuplicateFile",
            "Environment",
            "EventMapping",
            "Extension",
            "FileSFPCatalog",
            "Font",
            "IniFile",
            "IniLocator",
            "IsolatedComponent",
            "ListBox",
            "ListView",
            "LockPermissions",
            "MIME",
            "MoveFile",
            "MsiAssembly",
            "MsiAssemblyName",
            "MsiDigitalCertificate",
            "MsiDigitalSignature",
            "MsiPatchHeaders",
            "ODBCAttribute",
            "ODBCDataSource",
            "ODBCDriver",
            "ODBCSourceAttribute",
            "ODBCTranslator",
            "Patch",
            "PatchPackage",
            "ProgId",
            "PublishComponent",
            "RadioButton",
            "RemoveIniFile",
            "RemoveRegistry",
            "ReserveCost",
            "SFPCatalog",
            "SelfReg",
            "TextStyle",
            "TypeLib",
            "UIText",
            "Verb",
        ];
        for table in extra_tables {
            package.drop_table(table);
        }

        // let mut package = msi::Package::create(
        //     msi::PackageType::Installer,
        //     container,
        // )?;
        self.write_meta_info_to_package(&mut package, meta)?;
        self.write_tables_to_package(&mut package)?;
        self.write_cabinets_to_package(&mut package)?;

        info!("Finished building MSI");
        Ok(package)
    }

    pub(crate) fn write_meta_info_to_package<
        F: std::io::Read + std::io::Write + std::io::Seek,
    >(
        &self,
        package: &mut msi::Package<F>,
        meta: &MetaInformation,
    ) -> anyhow::Result<()> {
        let package_type = package.package_type();
        package.set_database_codepage(msi::CodePage::Windows1252);
        let summary_info = package.summary_info_mut();
        summary_info.set_codepage(msi::CodePage::Windows1252);
        summary_info.set_subject(meta.subject());

        if let Some(author) = meta.author() {
            summary_info.set_author(author);
        }
        summary_info.set_languages(meta.languages());
        if let Some(arch) = meta.architecture() {
            summary_info.set_arch(arch.to_string());
        }
        if let Some(comments) = meta.comments() {
            summary_info.set_comments(comments);
        }
        // TODO: Change this after testing. Just trying to make everything exactly the same.
        summary_info.set_creating_application("msitools 0.106");
        summary_info.set_creation_time_to_now();
        summary_info.set_last_save_time_to_now();
        summary_info.set_keywords(meta.keywords());
        summary_info.set_uuid(Uuid::new_v4());
        summary_info.set_word_count(2);
        // TODO: Determine if older versions should be supported.
        // Only support versions after 5.0
        summary_info.set_page_count(PageCount::_5_0 as i32);
        summary_info.set_doc_security(DocSecurity::from_package_type(
            &package_type,
        ) as i32);
        Ok(())
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
        // NOTE: The order in which the tables are written to matters. WTF.
        self.component.write_to_package(package)?;
        self.file.write_to_package(package)?;
        self.media.write_to_package(package)?;
        self.feature.write_to_package(package)?;
        self.feature_components.write_to_package(package)?;
        self.property.write_to_package(package)?;
        self.registry.write_to_package(package)?;
        self.msi_file_hash.write_to_package(package)?;
        // If this isn't the first sequence table to be filled, it is corrupted for some reason?
        self.admin_execute_sequence.write_to_package(package)?;
        self.admin_ui_sequence.write_to_package(package)?;
        self.advt_execute_sequence.write_to_package(package)?;
        self.install_execute_sequence.write_to_package(package)?;
        self.install_ui_sequence.write_to_package(package)?;
        self.service_control.write_to_package(package)?;
        self.service_install.write_to_package(package)?;
        self.msi_lock_permissions_ex.write_to_package(package)?;
        self.shortcut.write_to_package(package)?;

        // NOTE: Empty tables that seem to be required?
        self.signature.write_to_package(package)?;
        self.launch_condition.write_to_package(package)?;
        self.reg_locator.write_to_package(package)?;
        self.app_search.write_to_package(package)?;
        self.binary.write_to_package(package)?;
        self.custom_action.write_to_package(package)?;
        debug!(
            "Wrote tables to MSI: {:?}",
            package.tables().map(|t| t.name()).collect_vec()
        );
        Ok(())
    }

    pub(crate) fn write_cabinets_to_package<
        F: std::io::Read + std::io::Write + std::io::Seek,
    >(
        &self,
        package: &mut msi::Package<F>,
    ) -> anyhow::Result<()> {
        let previous_last_sequence = 1;
        for media in MsiTable::entries(&self.media)
            .iter()
            .sorted_by_key(|dao| Into::<i32>::into(*dao.last_sequence()))
        {
            let Some(cabinet_id) = media.cabinet_id() else {
                // Ignore media listings that don't represent an internal
                // cabinet file
                continue;
            };
            let last_sequence = Into::<i32>::into(*media.last_sequence());
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
            // Have to set the poisition of the file reader back to 0 so that it
            // gets read from the beginning when it gets read again.
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
            /// NOTE: From what I can tell attributes only need to be set on
            /// files in the File table as those attributes
            /// overwrite the attributes that are set in the cabinet
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
    pub fn add_to_tables(
        &mut self,
        dao: impl Into<MsiTableDao>,
    ) -> anyhow::Result<()> {
        let dao = Into::<MsiTableDao>::into(dao);
        match dao {
            MsiTableDao::Component(component_dao) => {
                IdentifierGeneratorTable::add(
                    &mut self.component,
                    component_dao,
                )
            }
            MsiTableDao::Directory(directory_dao) => {
                IdentifierGeneratorTable::add(
                    &mut self.directory,
                    directory_dao,
                )
            }
            MsiTableDao::File(file_dao) => {
                IdentifierGeneratorTable::add(&mut self.file, file_dao)
            }
            MsiTableDao::Registry(registry_dao) => {
                IdentifierGeneratorTable::add(&mut self.registry, registry_dao)
            }
            MsiTableDao::Feature(feature_dao) => {
                IdentifierGeneratorTable::add(&mut self.feature, feature_dao)
            }
            MsiTableDao::Property(dao) => self.property.add(dao),
            MsiTableDao::Media(dao) => self.media.add(dao),
            MsiTableDao::MsiFileHash(dao) => self.msi_file_hash.add(dao),
            MsiTableDao::FeatureComponents(dao) => {
                self.feature_components.add(dao)
            }
            _ => {
                todo!("Daos for {:?} are not implemented yet!", dao.table())
            }
        }
    }
}

impl Default for MsiBuilder {
    fn default() -> Self {
        let empty_entries = Rc::new(RefCell::new(Vec::new()));
        Self {
            meta: None,

            // Non-tables that need access to all or generate entity IDs.
            identifiers: empty_entries.clone(),
            cabinets: Cabinets::new(empty_entries.clone()),
            tables: Vec::new(),
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
