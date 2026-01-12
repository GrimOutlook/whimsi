use std::cell::RefCell;
use std::fs::File;
use std::io::Seek;
use std::path::Path;
use std::path::PathBuf;
use std::rc::Rc;
use std::str::FromStr;
use std::sync::LazyLock;

use anyhow::Context;
use anyhow::anyhow;
use anyhow::bail;
use getset::Getters;
use getset::Setters;
use itertools::Itertools;
use regex::Regex;
use tracing::debug;
use tracing::info;
use uuid::Uuid;

use crate::tables::admin_execute_sequence::table::AdminExecuteSequenceTable;
use crate::tables::admin_ui_sequence::table::AdminUiSequenceTable;
use crate::tables::advt_execute_sequence::table::AdvtExecuteSequenceTable;
use crate::tables::app_search::table::AppSearchTable;
use crate::tables::builder_list::MsiBuilderList;
use crate::tables::builder_table::MsiBuilderTable;
use crate::tables::component::dao::ComponentDao;
use crate::tables::component::table::ComponentIdentifier;
use crate::tables::component::table::ComponentTable;
use crate::tables::custom_action::table::CustomActionTable;
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
use crate::tables::icon::dao::IconDao;
use crate::tables::icon::table::IconIdentifier;
use crate::tables::icon::table::IconTable;
use crate::tables::id_generator_builder_list::IdGeneratorBuilderList;
use crate::tables::install_execute_sequence::table::InstallExecuteSequenceTable;
use crate::tables::install_ui_sequence::table::InstallUiSequenceTable;
use crate::tables::launch_condition::table::LaunchConditionTable;
use crate::tables::lock_permissions::dao::LockPermissionsDao;
use crate::tables::lock_permissions::lock_object::LockObject;
use crate::tables::lock_permissions::lock_permissions::LockPermissions;
use crate::tables::lock_permissions::table::LockPermissionsTable;
use crate::tables::media::cabinet_identifier::CabinetHandle;
use crate::tables::media::dao::MediaDao;
use crate::tables::media::table::MediaTable;
use crate::tables::meta::MetaInformation;
use crate::tables::msi_file_hash::dao::MsiFileHashDao;
use crate::tables::msi_file_hash::table::MsiFileHashTable;
use crate::tables::property::dao::PropertyDao;
use crate::tables::property::table::PropertyTable;
use crate::tables::reg_locator::table::RegLocatorTable;
use crate::tables::registry::table::RegistryTable;
use crate::tables::service_control::dao::ServiceControlDao;
use crate::tables::service_control::dao::ServiceControlIdentifier;
use crate::tables::service_control::event::Event;
use crate::tables::service_control::table::ServiceControlTable;
use crate::tables::service_install::dao::ServiceInstallDao;
use crate::tables::service_install::error_control::ErrorControl;
use crate::tables::service_install::service_type::ServiceType;
use crate::tables::service_install::start_type::StartType;
use crate::tables::service_install::table::ServiceInstallIdentifier;
use crate::tables::service_install::table::ServiceInstallTable;
use crate::tables::shortcut::dao::ShortcutDao;
use crate::tables::shortcut::table::ShortcutIdentifier;
use crate::tables::shortcut::table::ShortcutTable;
use crate::tables::signature::table::SignatureTable;
use crate::types::column::default_dir::DefaultDir;
use crate::types::column::filename::Filename;
use crate::types::column::formatted::Formatted;
use crate::types::column::identifier::Identifier;
use crate::types::column::identifier::ToIdentifier;
use crate::types::column::sequence::Sequence;
use crate::types::column::shortcut::Shortcut;
use crate::types::helpers::cabinet_info::CabinetInfo;
use crate::types::helpers::cabinets::Cabinets;
use crate::types::helpers::icon::IconInfo;
use crate::types::helpers::page_count::PageCount;
use crate::types::helpers::security_flag::DocSecurity;
use crate::types::properties::system_folder::SystemFolder;
use crate::types::standard_action::StandardAction;

/// An in-memory representation of the final MSI to be created.
#[derive(Clone, Debug, Getters, Setters)]
#[getset(get = "pub")]
pub struct MsiBuilder {
    /// Information about the whole package. Tracks both information for
    /// creating the MSI and information that is tracked in the
    /// _SummaryInformation table.
    ///
    /// WARN: The MSI cannot be built without this information being filled
    /// out.
    #[getset(set = "pub")]
    meta: Option<MetaInformation>,

    /// A list of all identifiers used in this MSI. Used to ensure no duplicate
    /// Identifiers are created.
    identifiers: Rc<RefCell<Vec<Identifier>>>,

    cabinets: Cabinets,
    icon_information: Vec<IconInfo>,

    component: ComponentTable,
    directory: DirectoryTable,
    file: FileTable,
    media: MediaTable,
    feature: FeatureTable,
    feature_components: FeatureComponentsTable,
    property: PropertyTable,
    registry: RegistryTable,
    msi_file_hash: MsiFileHashTable,
    admin_execute_sequence: AdminExecuteSequenceTable,
    admin_ui_sequence: AdminUiSequenceTable,
    advt_execute_sequence: AdvtExecuteSequenceTable,
    install_execute_sequence: InstallExecuteSequenceTable,
    install_ui_sequence: InstallUiSequenceTable,
    signature: SignatureTable,
    launch_condition: LaunchConditionTable,
    reg_locator: RegLocatorTable,
    app_search: AppSearchTable,
    custom_action: CustomActionTable,
    service_control: ServiceControlTable,
    service_install: ServiceInstallTable,
    shortcut: ShortcutTable,
    icon: IconTable,
    lock_permissions: LockPermissionsTable,
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
    ///   placed. Identifier should already be present in the `Directory` table
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
    /// let mut msi = MsiBuilder::default()
    ///     .with_path_contents(temp_dir_path, SystemFolder::ProgramFilesFolder).unwrap();
    ///
    /// // You will end up with the following on the windows install.
    /// // C:/ProgramFiles/
    /// // | - file1.txt
    /// // | child_dir1/
    /// // | child_dir2/
    /// //   | - file2.pdf
    ///
    /// // Print the structure so we can see what was made.
    /// msi.directory().print_directory_structure();
    ///
    /// assert_eq!(msi.directory().entries().len(), 4);
    /// assert_eq!(msi.file().entries().len(), 2);
    /// ```
    pub fn with_path_contents(
        mut self,
        path: impl Into<PathBuf>,
        parent: impl Into<DirectoryIdentifier>,
    ) -> anyhow::Result<Self> {
        self.add_path_contents(path, parent)?;
        Ok(self)
    }

    pub fn add_path_contents(
        &mut self,
        path: impl Into<PathBuf>,
        parent: impl Into<DirectoryIdentifier>,
    ) -> anyhow::Result<()> {
        let path = path.into();
        let parent = parent.into();
        debug!("Adding path [{:?}] contents to directory [{}]", path, parent);

        let directory_contents: Vec<std::fs::DirEntry> =
            std::fs::read_dir(&path)?.try_collect()?;
        for item in directory_contents {
            let filetype = item.file_type().unwrap_or_else(|_| {
                panic!("Failed to get file type for file {:?}", item)
            });
            let path = item.path();

            if filetype.is_file() {
                self.add_file_path(path, parent.clone())?;
            } else if filetype.is_dir() {
                let id = self.add_directory_from_path(&path, parent.clone())?;
                self.add_path_contents(path, id)?;
            } else {
                bail!("Create error for nonfile+nondir types")
            }
        }

        Ok(())
    }

    pub fn add_directory_from_path(
        &mut self,
        path: impl Into<PathBuf>,
        parent: impl Into<DirectoryIdentifier>,
    ) -> anyhow::Result<DirectoryIdentifier> {
        let path = path.into();

        let name = path
            .file_name()
            .with_context(|| format!(
                "Directory path [{path:?}] ended with `..` which is illegal."
            ))?
            .to_str()
            .with_context(|| format!("Directory path [{path:?}] has invalid unicode"))?;
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
        self.add_directory_actions();

        Ok(id)
    }

    pub fn add_directory_dao(
        &mut self,
        dao: DirectoryDao,
    ) -> anyhow::Result<()> {
        if let Some(parent) = dao.parent()
            && let Ok(system_folder) =
                SystemFolder::try_from(parent.to_identifier())
        {
            // Just ignore any errors when adding this directory since this will
            // likely already be in the table.
            let _ = self.add_directory_dao(system_folder.into());
        }
        IdGeneratorBuilderList::add(&mut self.directory, dao)
    }

    pub fn with_file_path(
        mut self,
        path: impl Into<PathBuf>,
        parent_id: impl Into<DirectoryIdentifier>,
    ) -> anyhow::Result<Self> {
        self.add_file_path(path, parent_id)?;
        Ok(self)
    }

    pub fn add_file_path(
        &mut self,
        path: impl Into<PathBuf>,
        parent_id: impl Into<DirectoryIdentifier>,
    ) -> anyhow::Result<()> {
        let path = path.into();
        debug!("Creating DAOs for {path:?}");

        let file_id = self.file.generate_id();
        let component_id = self.component.generate_id();
        let file_hash_dao = MsiFileHashDao::from_path(file_id.clone(), &path)?;
        let sequence = self.add_to_media(file_id.clone(), path.clone());
        let file_dao = FileDao::install_file_from_path(
            file_id.clone(),
            component_id.clone(),
            path,
            sequence?,
        )?;
        let component_dao =
            ComponentDao::new(component_id.clone(), parent_id.into())
                .with_keypath(file_id.into());

        self.add_to_tables(file_hash_dao)?;
        self.add_to_tables(file_dao)?;
        self.add_to_tables(component_dao)?;
        self.add_to_default_feature(&component_id)?;

        self.add_file_actions();
        Ok(())
    }

    /// Adds the given file to media so it will be installed when the MSI is
    /// run.
    ///
    /// If no media entry exists yet, one is created along with a cabinet file.
    fn add_to_media(
        &mut self,
        file_id: FileIdentifier,
        file_path: PathBuf,
    ) -> anyhow::Result<Sequence> {
        debug!("Adding file [{file_id}] with path [{file_path:?}] to media");
        // Verify there is a Media entry to add on to.
        if self.media.is_empty() {
            // Create a new cabinet file.
            let cabinet_id = self.new_cabinet();
            // Create a new media DAO.
            let dao = MediaDao::internal(1, cabinet_id.clone())
                .expect("Creating first entry to Media table failed");
            self.add_to_tables(dao)?;
        }

        let media_dao = self
            .media
            .get_last_internal_media_mut()
            .expect("Media table contains no internal CAB files");

        // Add the file to the cabinet.
        let cabinet_id = &media_dao
            .cabinet_id()
            .expect("Media DAO that had a cabinet ID apparently doesn't");
        let cabinet_info =
            self.cabinets.find_id_mut(cabinet_id).unwrap_or_else(|| panic!("Cabinet of ID [{}] referenced by media with disk ID [{}] was not found", cabinet_id, media_dao.disk_id()));
        cabinet_info.add_file(file_id, file_path);
        // Set the Media table entry's LastSequence to the number of files in
        // the cabinet.
        Ok(media_dao
            .set_last_sequence(cabinet_info)
            .expect("LastSequence got too large. TODO: Handle this case."))
    }

    /// Creates a new cabinet file and returns the ID.
    fn new_cabinet(&mut self) -> CabinetHandle {
        let id = self.cabinets.generate_id();
        self.cabinets
            .add_new(id.clone())
            .expect("Tried to insert cabinet of duplicate ID");
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
        ))?;
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

    // TODO: Make it so you don't have to define an icon since it's optional
    pub fn add_shortcut(
        &mut self,
        directory_id: DirectoryIdentifier,
        target: Shortcut,
        icon_path: &Path,
    ) -> anyhow::Result<ShortcutIdentifier> {
        let shortcut_id = self.shortcut.generate_id();
        let component_id = match target {
            Shortcut::Formatted(ref formatted) => {
                let (file_id, _directory_id) = self
                    .get_ids_of_path(&PathBuf::from_str(&formatted.to_string())?)
                    .ok_or(anyhow!("Directory and File entries couldn't be found for shortcut source path {formatted}"))?;
                self.file.entry_with_id(&file_id).unwrap().component().clone()
            }
            Shortcut::Identifier(ref feature_identifier) => {
                MsiBuilderList::entries(&self.feature_components)
                    .iter()
                    .find(|fc| *fc.feature() == *feature_identifier)
                    .unwrap()
                    .component()
                    .clone()
            }
        };
        let icon_id = self.add_icon(icon_path)?;
        self.add_to_tables(
            ShortcutDao::new(
                shortcut_id.clone(),
                directory_id,
                Filename::from_str(&shortcut_id.to_string()).unwrap_or_else(
                    |_| {
                        panic!(
                            "Shortcut
                        identifier {shortcut_id} cannot be used as a Filename"
                        )
                    },
                ),
                component_id,
                target,
            )
            .with_icon_(Some(icon_id))
            // Since we make a new icon stream for every icon the index will
            // always be 0
            .with_icon_index(Some(0)),
        )?;
        // Ignore duplicate errors
        self.add_shortcut_actions();
        Ok(shortcut_id)
    }

    pub fn add_icon(
        &mut self,
        icon_path: &Path,
    ) -> anyhow::Result<IconIdentifier> {
        let icon_id = self.icon.generate_id();
        self.add_to_tables(IconDao::new(icon_id.clone()))?;
        self.icon_information
            .push(IconInfo::new(icon_path.to_path_buf(), icon_id.clone()));
        Ok(icon_id)
    }

    pub fn add_service_install(
        &mut self,
        name: Formatted,
        service_type: ServiceType,
        start_type: StartType,
        error_control: ErrorControl,
        file_id: FileIdentifier,
        directory_id: DirectoryIdentifier,
    ) -> anyhow::Result<ServiceInstallIdentifier> {
        let service_install_id = self.service_install.generate_id();
        let component_id = self.component.generate_id();
        self.add_to_tables(
            ComponentDao::new(component_id.clone(), directory_id)
                .with_keypath(file_id.into()),
        )?;
        self.add_to_default_feature(&component_id)?;
        self.add_to_tables(ServiceInstallDao::new(
            service_install_id.clone(),
            name,
            service_type,
            start_type,
            error_control,
            component_id,
        ))?;
        self.add_service_actions();
        Ok(service_install_id)
    }

    // TODO: Add support for services not in the ServiceInstall table
    pub fn add_service_control(
        &mut self,
        service_install_id: &ServiceInstallIdentifier,
        name: Formatted,
        event: Event,
        wait: bool,
    ) -> anyhow::Result<ServiceControlIdentifier> {
        let component_id = self
            .service_install
            .entry_with_id(service_install_id)
            .unwrap()
            .component_()
            .clone();
        let service_control_id = self.service_control.generate_id();
        self.add_to_tables(ServiceControlDao::new(
            service_control_id.clone(),
            name,
            event,
            wait,
            component_id,
        ))?;
        self.add_service_actions();
        Ok(service_control_id)
    }

    pub fn add_lock_permissions(
        &mut self,
        lock_object: LockObject,
        user: Formatted,
        permission: LockPermissions,
    ) -> anyhow::Result<()> {
        self.add_to_tables(LockPermissionsDao::new(
            lock_object,
            user,
            permission,
        ))?;
        Ok(())
    }

    // TODO: Optimize this to only run once
    fn add_file_actions(&mut self) {
        let service_actions =
            vec![StandardAction::RemoveFiles, StandardAction::InstallFiles];
        let _ = self
            .install_execute_sequence
            .add_all(service_actions.into_iter().map_into().collect());
    }

    fn add_directory_actions(&mut self) {
        let service_actions =
            vec![StandardAction::RemoveFolders, StandardAction::CreateFolders];
        let _ = self
            .install_execute_sequence
            .add_all(service_actions.into_iter().map_into().collect());
    }

    // TODO: Optimize this to only run once
    fn add_shortcut_actions(&mut self) {
        let service_actions = vec![
            StandardAction::RemoveShortcuts,
            StandardAction::CreateShortcuts,
        ];
        let _ = self
            .install_execute_sequence
            .add_all(service_actions.into_iter().map_into().collect());
    }

    // TODO: Optimize this to only run once
    fn add_service_actions(&mut self) {
        let service_actions = vec![
            StandardAction::StopServices,
            StandardAction::DeleteServices,
            StandardAction::InstallServices,
            StandardAction::StartServices,
        ];
        let _ = self
            .install_execute_sequence
            .add_all(service_actions.into_iter().map_into().collect());
    }

    /// Build the MSI from all information given to MSIBuilder.
    pub fn build<F: std::io::Read + std::io::Write + std::io::Seek>(
        self,
        container: F,
    ) -> anyhow::Result<whimsi_msi::Package<F>> {
        let Some(ref meta) = self.meta else {
            bail!("Meta information cannot be blank");
        };
        info!("Building MSI");

        let mut package = whimsi_msi::Package::create(
            whimsi_msi::PackageType::Installer,
            container,
        )?;

        self.write_meta_info_to_package(&mut package, meta)?;
        self.write_tables_to_package(&mut package)?;
        self.write_icons_to_package(&mut package)?;
        self.write_cabinets_to_package(&mut package)?;

        info!("Finished building MSI");
        Ok(package)
    }

    pub(crate) fn write_meta_info_to_package<
        F: std::io::Read + std::io::Write + std::io::Seek,
    >(
        &self,
        package: &mut whimsi_msi::Package<F>,
        meta: &MetaInformation,
    ) -> anyhow::Result<()> {
        let package_type = package.package_type();
        package.set_database_codepage(whimsi_msi::CodePage::Windows1252);
        let summary_info = package.summary_info_mut();
        summary_info.set_codepage(whimsi_msi::CodePage::Windows1252);
        // TODO: Ensure that `subject` is the same as `ProductName` in the
        // `Property` table as specified [here](https://learn.microsoft.com/en-us/windows/win32/msi/subject-summary)
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
        // TODO: Change this after testing. Just trying to make everything
        // exactly the same.
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
        package: &mut whimsi_msi::Package<F>,
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
        // If this isn't the first sequence table to be filled, it is corrupted
        // for some reason?
        self.admin_execute_sequence.write_to_package(package)?;
        self.admin_ui_sequence.write_to_package(package)?;
        self.advt_execute_sequence.write_to_package(package)?;
        self.install_execute_sequence.write_to_package(package)?;
        self.install_ui_sequence.write_to_package(package)?;
        self.service_control.write_to_package(package)?;
        self.service_install.write_to_package(package)?;
        // WARN: LockPermissions table is causing the MSI to be
        // unreadable. `reference` example installs just fine with it commented
        // out. Seems like the command `example.ron` for the commandline
        // fails to install though even with it and everything from
        // service_control down to icon commented out. Haven't tried
        // commenting out any more but my guess is it's a data
        // formatting issue since there are a different number of directories in
        // the table between the `reference` example msi and the
        // `example.ron` msi.
        //
        // write_to_package(package)?;
        self.shortcut.write_to_package(package)?;
        self.icon.write_to_package(package)?;

        // NOTE: Empty tables that seem to be required?
        self.signature.write_to_package(package)?;
        self.launch_condition.write_to_package(package)?;
        self.reg_locator.write_to_package(package)?;
        self.app_search.write_to_package(package)?;
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
        package: &mut whimsi_msi::Package<F>,
    ) -> anyhow::Result<()> {
        let previous_last_sequence = 1;
        for media in MsiBuilderTable::entries(&self.media)
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
            if files.is_empty() {
                unreachable!(
                    "No files found for given cabinet file. This should not happen."
                )
            }

            let mut cabinet_file = self.create_cabinet_file(cabinet_info)?;
            // Have to set the position of the file reader back to 0 so that it
            // gets read from the beginning when it gets read again.
            cabinet_file.rewind().expect("Failed to rewind cabinet file");

            self.write_cabinet_to_package(
                cabinet_info,
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
        let folder = cab_builder.add_folder(cab::CompressionType::MsZip);
        cabinet_info.files().iter().for_each(|file| {
            // NOTE: From what I can tell attributes only need to be set on
            // files in the File table as those attributes
            // overwrite the attributes that are set in the cabinet
            // file.
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
        package: &mut whimsi_msi::Package<F>,
    ) -> anyhow::Result<()> {
        let cabinet_id = cabinet_info.id();
        debug!(
            "Writing cabinet file for cabinet ID [{}] to package",
            cabinet_id
        );
        let mut writer = package
            .write_stream(&cabinet_id.to_string())
            .context("Failed to create cabinet stream writier for package")?;
        std::io::copy(cabinet, &mut writer).with_context(|| {
            format!("Failed to copy cabinet [{cabinet_id}] to package")
        })?;

        Ok(())
    }

    // Icon stream writing information can be found partially
    // [here](https://learn.microsoft.com/en-us/windows/win32/msi/ole-limitations-on-streams) in
    // the first limitation listed
    pub(crate) fn write_icons_to_package<
        F: std::io::Read + std::io::Write + std::io::Seek,
    >(
        &self,
        package: &mut whimsi_msi::Package<F>,
    ) -> anyhow::Result<()> {
        for icon in &self.icon_information {
            let mut icon_data = File::open(icon.path()).with_context(|| {
                format!("Failed to open icon at path {:?}", icon.path())
            })?;
            let icon_id = icon.identifier().to_string();
            // Stream name for Icons is defined
            // [here](https://learn.microsoft.com/en-us/windows/win32/msi/ole-limitations-on-streams) in
            // in the first limitation listed
            let stream_name = [self.icon.name(), ".", &icon_id].concat();

            let mut writer = package
                .write_stream(&stream_name)
                .context("Failed to create icon stream writer for package")?;
            std::io::copy(&mut icon_data, &mut writer).with_context(|| {
                format!(
                    "Failed to copy icon stream data [{icon_id}] to package"
                )
            })?;
        }
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
    fn add_to_tables(&mut self, dao: impl Into<Dao>) -> anyhow::Result<()> {
        let dao = Into::<Dao>::into(dao);
        match dao {
            Dao::Component(dao) => {
                debug!(
                    "Adding component_id {} with directory_id {} to MSI",
                    dao.component(),
                    dao.directory()
                );
                IdGeneratorBuilderList::add(&mut self.component, dao)
            }
            Dao::Directory(dao) => {
                debug!(
                    "Adding directory_id {} with name {} to MSI",
                    dao.directory(),
                    dao.default_dir()
                );
                IdGeneratorBuilderList::add(&mut self.directory, dao)
            }
            Dao::File(dao) => {
                debug!(
                    "Adding file id {} with name {} to MSI",
                    dao.file(),
                    dao.name()
                );
                IdGeneratorBuilderList::add(&mut self.file, dao)
            }
            Dao::Registry(dao) => {
                debug!(
                    "Adding registry id {} with path {:?} to MSI",
                    dao.registry(),
                    dao.name()
                );
                IdGeneratorBuilderList::add(&mut self.registry, dao)
            }
            Dao::Feature(dao) => {
                debug!(
                    "Adding feature id {} with name {:?} to MSI",
                    dao.feature(),
                    dao.title()
                );
                IdGeneratorBuilderList::add(&mut self.feature, dao)
            }
            Dao::Shortcut(dao) => {
                debug!(
                    "Adding shortcut id {} with target {:?} to MSI",
                    dao.identifier(),
                    dao.target()
                );
                IdGeneratorBuilderList::add(&mut self.shortcut, dao)
            }
            Dao::ServiceInstall(dao) => {
                debug!(
                    "Adding service install id {} with name {} to MSI",
                    dao.identifier(),
                    dao.name()
                );
                IdGeneratorBuilderList::add(&mut self.service_install, dao)
            }
            Dao::ServiceControl(dao) => {
                debug!(
                    "Adding service control id {} with name {} to MSI",
                    dao.service_control(),
                    dao.name()
                );
                IdGeneratorBuilderList::add(&mut self.service_control, dao)
            }
            Dao::Icon(dao) => {
                debug!("Adding icon id {} to MSI", dao.name());
                IdGeneratorBuilderList::add(&mut self.icon, dao)
            }
            Dao::Property(dao) => {
                debug!(
                    "Adding property id {} with value {} to MSI",
                    dao.property(),
                    dao.value()
                );
                self.property.add(dao)
            }
            Dao::Media(dao) => {
                debug!("Adding media id {} to MSI", dao.disk_id());
                self.media.add(dao)
            }
            Dao::MsiFileHash(dao) => {
                debug!("Adding file hash for file id {} to MSI", dao.file());
                self.msi_file_hash.add(dao)
            }
            Dao::FeatureComponents(dao) => {
                debug!(
                    "Adding component id {} to feature id {} in MSI",
                    dao.component(),
                    dao.feature()
                );
                self.feature_components.add(dao)
            }
            Dao::LockPermissions(dao) => {
                debug!(
                    "Adding lock permission to target id {:?} in MSI",
                    dao.lock_object()
                );
                self.lock_permissions.add(dao)
            }
        }
    }

    pub fn file_entries_with_directory_id(
        &self,
        directory_id: &DirectoryIdentifier,
    ) -> Vec<&FileDao> {
        self.file
            .entries()
            .iter()
            .filter(|file| {
                self.component()
                    .entry_with_id(file.component())
                    .expect(
                        "Component ID referenced by FileDAO is not present in Component table",
                    )
                    .directory()
                    == directory_id
            })
            .collect_vec()
    }

    fn get_ids_of_path(
        &self,
        service_path: &Path,
    ) -> Option<(FileIdentifier, DirectoryIdentifier)> {
        let parent_dir = service_path.parent()?;
        let directory_id = self.get_directory_id_of_path(parent_dir)?;
        let executable_file = service_path.file_name()?;
        let file_id = self
            .file_entries_with_directory_id(&directory_id)
            .iter()
            .find(|file_dao| {
                file_dao.name().to_string() == executable_file.to_string_lossy()
            })?
            .file()
            .clone();

        Some((file_id, directory_id))
    }

    fn get_directory_id_of_path(
        &self,
        path: &Path,
    ) -> Option<DirectoryIdentifier> {
        let last_component =
            self.get_last_component(path.to_string_lossy().as_ref())?;
        if let Ok(system_folder) = SystemFolder::from_str(&last_component) {
            return Some(system_folder.to_identifier().into());
        }

        self.directory()
            .entry_with_name(&DefaultDir::Filename(
                Filename::from_str(&last_component).unwrap(),
            ))
            .map(|d| d.directory().clone())
    }

    fn get_last_component(&self, path: &str) -> Option<String> {
        let last_component = self
            .get_component_value(
                PathBuf::from_str(path)
                    .ok()?
                    .components()
                    .next_back()
                    .unwrap_or_else(|| {
                        panic!("Path {path} doesn't have any components")
                    })
                    .as_os_str()
                    .to_string_lossy()
                    .to_string(),
            )
            .ok()?
            .last()
            .unwrap()
            .to_string();
        Some(last_component)
    }

    fn get_component_value(
        &self,
        component: String,
    ) -> anyhow::Result<Vec<String>> {
        static CUSTOM_PROPERTY: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r"\[([A-Za-z0-9_]+)\]").unwrap());

        let value = if let Some(system_folder) =
            Self::get_reserved_property(&component)
        {
            vec![system_folder]
        } else if let Some(captures) = CUSTOM_PROPERTY.captures(&component)
            && let Some(property) = captures.get(1)
        {
            PathBuf::from_str(
                &self
                    .property
                    .get(property.as_str())
                    .unwrap_or_else(|| {
                        panic!("Failed to get property: {}", property.as_str())
                    })
                    .to_string(),
            )?
            .components()
            .map(|c| {
                self.get_component_value(
                    c.as_os_str().to_string_lossy().to_string(),
                )
            })
            .flatten_ok()
            .collect::<anyhow::Result<Vec<String>>>()?
        } else {
            vec![component]
        };
        Ok(value)
    }

    fn get_reserved_property(prop: &str) -> Option<String> {
        static RESERVED_PROPERTY: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r"\[\[([A-Za-z0-9_]+)\]\]").unwrap());
        if let Some(captures) = RESERVED_PROPERTY.captures(prop)
            && let Some(property) = captures.get(1)
        {
            let Ok(system_folder) = SystemFolder::from_str(property.as_str())
            else {
                panic!(
                    "Property is not a valid reserved property: {}",
                    property.as_str()
                );
            };
            Some(system_folder.to_string())
        } else {
            None
        }
    }
}

impl Default for MsiBuilder {
    fn default() -> Self {
        let empty_entries = Rc::new(RefCell::new(Vec::new()));
        Self {
            meta: None,

            // Non-table trackers
            icon_information: Default::default(),

            // Non-tables that need access to all or generate entity IDs.
            identifiers: empty_entries.clone(),
            cabinets: Cabinets::new(empty_entries.clone()),

            // Tables that don't have IDs for their entries.
            property: Default::default(),
            media: Default::default(),
            feature_components: Default::default(),
            msi_file_hash: Default::default(),
            admin_execute_sequence: Default::default(),
            admin_ui_sequence: Default::default(),
            advt_execute_sequence: Default::default(),
            install_execute_sequence: Default::default(),
            install_ui_sequence: Default::default(),
            launch_condition: Default::default(),
            reg_locator: Default::default(),
            app_search: Default::default(),
            custom_action: Default::default(),
            lock_permissions: Default::default(),

            // Tables that can generate IDs for their entries and the IDs must
            // be unique across the MSI.
            component: ComponentTable::new(empty_entries.clone()),
            directory: DirectoryTable::new(empty_entries.clone()),
            feature: FeatureTable::new(empty_entries.clone()),
            file: FileTable::new(empty_entries.clone()),
            registry: RegistryTable::new(empty_entries.clone()),
            service_install: ServiceInstallTable::new(empty_entries.clone()),
            service_control: ServiceControlTable::new(empty_entries.clone()),
            shortcut: ShortcutTable::new(empty_entries.clone()),
            icon: IconTable::new(empty_entries.clone()),
            signature: SignatureTable::new(empty_entries.clone()),
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
