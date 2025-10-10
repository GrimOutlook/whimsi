pub(crate) mod builder_table;
pub(crate) mod dao;
pub(crate) mod identifier_generator_table;
pub mod meta;

use crate as whimsi_lib;
use crate::tables::builder_table::MsiTableKind;
use crate::tables::dao::MsiDao;
use crate::tables::identifier_generator_table::IdentifierGeneratorTable;
use crate::types::column::binary::Binary;
use crate::types::column::condition::Condition;
use crate::types::column::custom_source::CustomSource;
use crate::types::column::default_dir::DefaultDir;
use crate::types::column::double_integer::DoubleInteger;
use crate::types::column::filename::Filename;
use crate::types::column::formatted::Formatted;
use crate::types::column::guid::Guid;
use crate::types::column::integer::Integer;
use crate::types::column::language::Language;
use crate::types::column::property::Property;
use crate::types::column::reg_path::RegPath;
use crate::types::column::sequence::Sequence;
use crate::types::column::shortcut::Shortcut;
use crate::types::column::text::Text;
use crate::types::column::version::Version;
use crate::types::helpers::action_identifier::ActionIdentifier;
use crate::types::helpers::attributes::component::ComponentAttributes;
use crate::types::helpers::attributes::feature::FeatureAttributes;
use crate::types::helpers::attributes::file::FileAttributes;
use crate::types::helpers::cabinets::CabinetIdentifier;
use crate::types::helpers::custom_action_type::CustomActionType;
use crate::types::helpers::date::Date;
use crate::types::helpers::disk_id::DiskId;
use crate::types::helpers::error_control::ErrorControl;
use crate::types::helpers::key_path::KeyPath;
use crate::types::helpers::last_sequence::LastSequence;
use crate::types::helpers::locator_type::LocatorType;
use crate::types::helpers::lock_object::LockObject;
use crate::types::helpers::lock_permission::LockPermission;
use crate::types::helpers::lock_table::LockTable;
use crate::types::helpers::primary_identifier::PrimaryIdentifier;
use crate::types::helpers::registry_root::RegistryRoot;
use crate::types::helpers::service_control_event::ServiceControlEvent;
use crate::types::helpers::service_type::ServiceType;
use crate::types::helpers::show_cmd::ShowCmd;
use crate::types::helpers::start_type::StartType;
use crate::types::standard_action::{AdvtAction, StandardAction};

pub struct MsiTables {
    inner: Vec<MsiTable>,
}

// TODO: Look at Directory to see the form that I eventually want to have implemented.
whimsi_table_macro::msi_table_list! {
    /// [*Reference*](https://learn.microsoft.com/en-us/windows/win32/msi/database-tables)
    enum MsiTable {
        // TODO: ActionTable,

        /// Lists ADMIN actions in sequence.
        ///
        /// The AdminExecuteSequence table lists actions that the installer calls in sequence when
        /// the top-level ADMIN action is executed.
        ///
        /// ADMIN actions in the install sequence, up to the InstallValidate action and any exit
        /// dialog boxes, are located in the AdminUISequence table.
        ///
        /// ADMIN actions from the InstallValidate action through the end of the install sequence
        /// are in the AdminExecuteSequence table. Because the AdminExecuteSequence table needs to
        /// stand alone, it also contains any necessary initialization actions such as
        /// LaunchConditions, CostInitialize, FileCost, and CostFinalize.
        ///
        /// Custom actions requiring a user interface should use MsiProcessMessage instead of
        /// authored dialog boxes created using the Dialog table.
        ///
        /// [*Reference*](https://learn.microsoft.com/en-us/windows/win32/msi/adminexecutesequence-table)
        AdminExecuteSequence {
            #[msi_column(primary_key, category = msi::Category::Identifier, length = 72)]
            action: ActionIdentifier,
            #[msi_column(category = msi::Category::Condition, length = 255)]
            condition: Option<Condition>,
            #[msi_column(category = msi::Category::Condition, length = 255)]
            sequence: Option<Integer>,
        },

        /// Lists UI ADMIN actions in sequence.
        ///
        /// The AdminUISequence table lists actions that the installer calls in sequence when the
        /// top-level ADMIN action is executed and the internal user interface level is set to full
        /// UI or reduced UI. The installer skips the actions in this table if the user interface
        /// level is set to basic UI or no UI. See About the User Interface.
        ///
        /// ADMIN actions in the install sequence up to the InstallValidate action, and any exit
        /// dialog boxes, are located in the AdminUISequence table. All actions from the
        /// InstallValidate through the end of the install sequence are in the AdminExecuteSequence
        /// table. Because the AdminExecuteSequence table needs to stand alone, it also contains
        /// any necessary initialization actions such as LaunchConditions, CostInitialize,
        /// FileCost, and CostFinalize. It also has the ExecuteAction action.
        ///
        /// [*Reference*](https://learn.microsoft.com/en-us/windows/win32/msi/adminuisequence-table)
        AdminUiSequence {
            #[msi_column(primary_key, category = msi::Category::Identifier, length = 72)]
            action: ActionIdentifier,
            #[msi_column(category = msi::Category::Condition, length = 255)]
            condition: Option<Condition>,
            #[msi_column(category = msi::Category::Condition, length = 255)]
            sequence: Option<Integer>,
        },

        /// Lists ADVERTISE actions in sequence.
        ///
        /// The AdvtExecuteSequence table lists actions the installer calls when the top-level
        /// ADVERTISE action is executed.
        ///
        /// [*Reference*](https://learn.microsoft.com/en-us/windows/win32/msi/advtexecutesequence-table)
        AdvtExecuteSequence {
            #[msi_column(primary_key, category = msi::Category::Identifier, length = 72)]
            action: AdvtAction,
            #[msi_column(category = msi::Category::Condition, length = 255)]
            condition: Option<Condition>,
            #[msi_column(category = msi::Category::Condition, length = 255)]
            sequence: Option<Integer>,
        },

        // TODO: AppId,

        /// Lists properties used to search by file signature.
        ///
        /// The AppSearch table contains properties needed to search for a file having a particular
        /// file signature. The AppSearch table can also be used to set a property to the existing
        /// value of a registry or .ini file entry.
        ///
        /// [*Reference*](https://learn.microsoft.com/en-us/windows/win32/msi/appsearch-table)
        AppSearch {
            /// Running the AppSearch action sets this property to the location of the file
            /// indicated by the Signature_ column. This property is set if the file signature
            /// exists on the user's computer. The properties used in this column must be public
            /// properties and have an identifier that contains no lowercase letters.
            ///
            /// The property listed in the Property field may be initialized in the Property table
            /// or from a command line. If the AppSearch action locates the signature, the
            /// installer overrides the initialized property value with the found value. If the
            /// signature is not found, then the initial property value is used. If the property
            /// was never initialized, then the property will only be set if the signature is
            /// found. Otherwise, the property is undefined.
            #[msi_column(primary_key, category = msi::Category::Identifier, length = 72)]
            property: Identifier,

            /// The Signature_ column contains a unique identifier called a signature and is also
            /// an external key into the RegLocator, IniLocator, CompLocator, and DrLocator tables.
            /// When searching for a file, the value in this column must also be a foreign key into
            /// the Signature table. If the value in this column is not listed in the Signature
            /// table, the installer determines that the search is for a directory.
            #[msi_column(identifier(foreign_key = "Signature"), category = msi::Category::Identifier, length = 72)]
            signature_: SignatureIdentifier,
        },

        // TODO: BBControl,
        // TODO: Billboard,

        /// Holds binary data for bitmaps and icons.
        ///
        /// The Binary table holds the binary data for items such as bitmaps, animations, and
        /// icons. The binary table is also used to store data for custom actions. See [OLE
        /// Limitations](https://learn.microsoft.com/en-us/windows/win32/msi/ole-limitations-on-streams)
        /// on Streams.
        ///
        /// [*Reference*](https://learn.microsoft.com/en-us/windows/win32/msi/binary-table)
        Binary {
            #[msi_column(primary_key, identifier(generated), category = msi::Category::Identifier, length = 72)]
            name: BinaryIdentifier,
            #[msi_column(category = msi::Category::Binary, length = 0)]
            data: Binary,
        },

        // TODO: BindImage,
        // TODO: CCPSearch,
        // TODO: CheckBox,
        // TODO: Class,
        // TODO: CoimboBox,
        // TODO: CompLocators,
        // TODO: Complus,

        /// Lists installation components.
        ///
        /// [*Reference*](https://learn.microsoft.com/en-us/windows/win32/msi/component-table)
        Component {
            #[msi_column(primary_key, identifier(generated), category = msi::Category::Identifier, length = 72)]
            component: ComponentIdentifier,
            #[msi_column(category = msi::Category::Guid, length = 38)]
            component_id: Option<Guid>,
            #[msi_column(identifier(foreign_key = "Directory"), category = msi::Category::Identifier, length = 72)]
            directory_: DirectoryIdentifier,
            #[msi_column(category = msi::Category::Integer)]
            attributes: ComponentAttributes,
            #[msi_column(category = msi::Category::Condition, length = 255)]
            condition: Option<Condition>,
            #[msi_column(category = msi::Category::Identifier, length = 72)]
            key_path: Option<KeyPath>,
        },

        // TODO: Condition,
        // TODO: Control,
        // TODO: ControlCondition,
        // TODO: ControlEvent,
        // TODO: CreateFolder,

        /// Integrates custom actions into the installation.
        ///
        /// The CustomAction table provides the means of integrating custom code and data into the
        /// installation. The source of the code that is executed can be a stream contained within
        /// the database, a recently installed file, or an existing executable file.
        ///
        /// [*Reference*](https://learn.microsoft.com/en-us/windows/win32/msi/customaction-table)
        CustomAction {
            /// Name of the action. The action normally appears in a sequence table unless it is
            /// called by another custom action. If the name matches any built-in action, then the
            /// custom action is never called.
            #[msi_column(primary_key, category = msi::Category::Identifier, length = 72)]
            action: ActionIdentifier,
            #[msi_column(column_name = "Type", category = msi::Category::Integer)]
            typ: CustomActionType,
            #[msi_column(category = msi::Category::CustomSource, length = 72)]
            source: Option<CustomSource>,
            #[msi_column(category = msi::Category::Formatted, length = 255)]
            target: Option<Formatted>,
            /// Not supported currently because it is extremely niche.
            ///
            /// Enter the msidbCustomActionTypePatchUninstall value in this field to specify a
            /// custom action with the Custom Action Patch Uninstall Option.
            #[msi_column(category = msi::Category::DoubleInteger)]
            extended_type: Option<Integer>,
        },

        // TODO: Dialog,

        /// Directory layout for the application
        ///
        /// [*Reference*](https://learn.microsoft.com/en-us/windows/win32/msi/directory-table)
        Directory {
            #[msi_column(primary_key, identifier(generated), category = msi::Category::Identifier, length = 72)]
            directory: DirectoryIdentifier,
            #[msi_column(identifier(foreign_key = "Directory"), column_name = "Directory_Parent", category = msi::Category::Identifier, length = 72)]
            parent_directory: Option<DirectoryIdentifier>,
            #[msi_column(localizable, category = msi::Category::DefaultDir, length = 255)]
            default_dir: DefaultDir,
        },

        // TODO: DrLocator,
        // TODO: DuplicateFile,
        // TODO: Environment,
        // TODO: Error,
        // TODO: EventMapping,
        // TODO: Extensiuon,

        /// Defines the logical tree structure of features.
        ///
        /// [*Reference*](https://learn.microsoft.com/en-us/windows/win32/msi/feature-table)
        Feature {
            // Yes I know it's weird that features have a different Identifier length, but it is
            // explicitly stated [here](https://learn.microsoft.com/en-us/windows/win32/msi/feature-table#feature).
            #[msi_column(primary_key, identifier(generated), category = msi::Category::Identifier, length = 38)]
            feature: FeatureIdentifier,
            #[msi_column(identifier(foreign_key = "Feature"), column_name = "Feature_Parent", category = msi::Category::Identifier, length = 38)]
            parent_feature: Option<FeatureIdentifier>,
            #[msi_column(localizable, category = msi::Category::Text, length = 64)]
            title: Option<Text>,
            #[msi_column(localizable, category = msi::Category::Text, length = 255)]
            description: Option<Text>,
            #[msi_column(category = msi::Category::Integer)]
            display: Option<Integer>,
            #[msi_column(category = msi::Category::Integer)]
            level: Integer,
            #[msi_column(identifier(foreign_key = "Directory"), category = msi::Category::Identifier, length = 72)]
            directory_: Option<DirectoryIdentifier>,
            #[msi_column(category = msi::Category::Integer)]
            attributes: FeatureAttributes,

        },

        /// Defines features and component relationships.
        ///
        /// [*Reference*](https://learn.microsoft.com/en-us/windows/win32/msi/featurecomponents-table)
        FeatureComponents {
            #[msi_column(primary_key, identifier(foreign_key = "Feature"), category = msi::Category::Identifier, length = 72)]
            feature_: FeatureIdentifier,
            #[msi_column(primary_key, identifier(foreign_key = "Component"), category = msi::Category::Identifier, length = 72)]
            component_: ComponentIdentifier,
        },

        /// Complete list of source files with their attributes.
        ///
        /// [*Reference*](https://learn.microsoft.com/en-us/windows/win32/msi/file-table)
        File {
            #[msi_column(primary_key, identifier(generated), category = msi::Category::Identifier, length = 72)]
            file: FileIdentifier,
            #[msi_column(identifier(foreign_key = "Component"), category = msi::Category::Identifier, length = 72)]
            component_: ComponentIdentifier,
            #[msi_column(localizable, category = msi::Category::Filename, length = 255)]
            file_name: Filename,
            #[msi_column(category = msi::Category::DoubleInteger)]
            file_size: DoubleInteger,
            #[msi_column(category = msi::Category::Version, length = 72)]
            version: Option<Version>,
            #[msi_column(category = msi::Category::Language, length = 20)]
            language: Option<Language>,
            #[msi_column(category = msi::Category::Integer)]
            attributes: Option<FileAttributes>,
            #[msi_column(category = msi::Category::Integer)]
            sequence: Sequence,
        },

        // TODO: FileSFPCatqalog,
        // TODO: Font,

        /// Contains the icon files.
        ///
        /// [*Reference*](https://learn.microsoft.com/en-us/windows/win32/msi/icon-table)
        Icon {
            #[msi_column(primary_key, identifier(generated), category = msi::Category::Identifier, length = 72)]
            name: IconIdentifier,
            #[msi_column(category = msi::Category::Binary, length = 0)]
            data: Binary,
        },

        // TODO: IniFile,
        // TODO: IniLocator,


        /// Lists INSTALL actions in sequence.
        ///
        /// The InstallExecuteSequence table lists actions that are executed when the top-level
        /// INSTALL action is executed.
        ///
        /// Actions in the install sequence up to the InstallValidate action, and any exit dialog boxes,
        /// are located in the InstallUISequence table. All actions from the InstallValidate through the
        /// end of the install sequence are in the InstallExecuteSequence table. Because the
        /// InstallExecuteSequence table needs to stand alone, it has any necessary initialization actions
        /// such as the LaunchConditions, CostInitialize, FileCost, and CostFinalize actions.
        ///
        /// Custom actions requiring a user interface should use MsiProcessMessage instead of authored
        /// dialog boxes created using the Dialog table.
        ///
        /// [*Reference*](https://learn.microsoft.com/en-us/windows/win32/msi/installexecutesequence-table)
        InstallExecuteSequence {
            #[msi_column(primary_key, category = msi::Category::Identifier, length = 72)]
            action: ActionIdentifier,
            #[msi_column(category = msi::Category::Condition, length = 255)]
            condition: Option<Condition>,
            #[msi_column(category = msi::Category::Condition, length = 255)]
            sequence: Option<Integer>,
        },

        /// Lists UI INSTALL actions in sequence.
        ///
        ///
        /// The InstallUISequence table lists actions that are executed when the top-level INSTALL
        /// action is executed and the internal user interface level is set to full UI or reduced
        /// UI. The installer skips the actions in this table if the user interface level is set to
        /// basic UI or no UI. See About the User Interface.
        ///
        /// Actions in the install sequence up to the InstallValidate action, and the exit dialog
        /// boxes, are located in the InstallUISequence table. All actions from the InstallValidate
        /// through the end of the install sequence are in the InstallExecuteSequence table.
        /// Because the InstallExecuteSequence table needs to stand alone, it has any necessary
        /// initialization actions such as the LaunchConditions, CostInitialize, FileCost, and the
        /// CostFinalize, and ExecuteAction action.
        ///
        /// [*Reference*](https://learn.microsoft.com/en-us/windows/win32/msi/installuisequence-table)
        InstallUiSequence {
            #[msi_column(primary_key, category = msi::Category::Identifier, length = 72)]
            action: ActionIdentifier,
            #[msi_column(category = msi::Category::Condition, length = 255)]
            condition: Option<Condition>,
            #[msi_column(category = msi::Category::Condition, length = 255)]
            sequence: Option<Integer>,
        },


        // IsolatedComponent,

        /// Lists conditions for the installation to begin.
        ///
        /// The LaunchCondition table is used by the LaunchConditions action. It contains a list of
        /// conditions that all must be satisfied for the installation to begin.
        ///
        /// [*Reference*](https://learn.microsoft.com/en-us/windows/win32/msi/launchcondition-table)
        LaunchCondition {
            #[msi_column(primary_key, category = msi::Category::Condition, length = 255)]
            condition: Condition,
            #[msi_column(localizable, category = msi::Category::Formatted, length = 255)]
            description: Option<Formatted>,
        },

        // TODO: ListBox,
        // TODO: ListView,


        /// Defines locked-down portions of the application.
        ///
        /// [*Reference*](https://learn.microsoft.com/en-us/windows/win32/msi/lockpermissions-table)
        LockPermissions {
            #[msi_column(primary_key, category = msi::Category::Identifier, length = 72)]
            lock_object: LockObject,
            #[msi_column(primary_key, category = msi::Category::Text, length = 32)]
            table: LockTable,
            #[msi_column(primary_key, category = msi::Category::Formatted, length = 255)]
            domain: Option<Formatted>,
            #[msi_column(primary_key, category = msi::Category::Formatted, length = 255)]
            user: Formatted,
            #[msi_column(category = msi::Category::DoubleInteger)]
            permission: LockPermission
        },

        /// Lists source media disks for the installation.
        ///
        /// The Media table describes the set of disks that make up the source media for the installation.
        ///
        /// [*Reference*](https://learn.microsoft.com/en-us/windows/win32/msi/media-table)
        Media {
            #[msi_column(primary_key, category = msi::Category::Integer)]
            disk_id: DiskId,
            #[msi_column(category = msi::Category::Integer)]
            last_sequence: LastSequence,
            #[msi_column(localizable, category = msi::Category::Text, length = 64)]
            disk_prompt: Text,
            #[msi_column(category = msi::Category::Cabinet, length = 255)]
            cabinet: CabinetIdentifier,
            #[msi_column(category = msi::Category::Text, length = 32)]
            volume_label: Text,
            #[msi_column(category = msi::Category::Property, length = 72)]
            source: Property,
        },

        // TODO: MIME,
        // TODO: MoveFile,
        // TODO: MsiAssembly,
        // TODO: MsiAssemblyName,
        // TODO: MsiDigitalCertificate,
        // TODO: MsiDigitalSiganture,
        // TODO: MsiEmbeddedChainer,

        /// Stores a 128-bit hash of source files provided by the Windows Installer package.
        ///
        /// [*Reference*](https://learn.microsoft.com/en-us/windows/win32/msi/msifilehash-table)
        MsiFileHash {
            #[msi_column(primary_key, identifier(foreign_key = "File"), category = msi::Category::Identifier, length = 72)]
            file_: FileIdentifier,
            #[msi_column(category = msi::Category::Integer)]
            options: Integer,
            #[msi_column(category = msi::Category::DoubleInteger)]
            hash_part_1: DoubleInteger,
            #[msi_column(category = msi::Category::DoubleInteger)]
            hash_part_2: DoubleInteger,
            #[msi_column(category = msi::Category::DoubleInteger)]
            hash_part_3: DoubleInteger,
            #[msi_column(category = msi::Category::DoubleInteger)]
            hash_part_4: DoubleInteger,
        },

        // TODO: MsiFileHash,
        // TODO: MsiLockFilePermissionsEx,
        // TODO: MsiPackageCertificate,
        // TODO: MsiPatchCertificate,
        // TODO: MsiPatchHeaders,
        // TODO: MsiPatchMetadata,
        // TODO: MsiPatchOldAssemblyName,
        // TODO: MsiPatchOldAssemblyFile,
        // TODO: MsiPatchSequence,
        // TODO: MsiServiceConfig,
        // TODO: MsiServiceConfigFailureActions,
        // TODO: MsiSFFCBypass,
        // TODO: ODBCAttribute,
        // TODO: ODBCDataSource,
        // TODO: ODBCDrive,
        // TODO: ODBCSourceAttribute,
        // TODO: ODBCTranslator,
        // TODO: Patch,
        // TODO: PatchPackage,
        // TODO: ProgId,

        /// Lists property names and values for all properties.
        ///
        /// [*Reference*](https://learn.microsoft.com/en-us/windows/win32/msi/property-table)
        Property {
            #[msi_column(primary_key, identifier(), category = msi::Category::Identifier, length = 72)]
            property: PropertyIdentifier,
            #[msi_column(localizable, category = msi::Category::Text, length = 0)]
            value: Text,
        },

        // TODO: PublishComponent,
        // TODO: RadioButton,

        /// Lists registry information for the application.
        ///
        /// [*Reference*](https://learn.microsoft.com/en-us/windows/win32/msi/registry-table)
        Registry {
            #[msi_column(primary_key, identifier(generated), category = msi::Category::Identifier, length = 72)]
            registry: RegistryIdentifier,
            #[msi_column(category = msi::Category::Integer)]
            root: RegistryRoot,
            #[msi_column(localizable, category = msi::Category::RegPath, length = 255)]
            key: RegPath,
            #[msi_column(localizable, category = msi::Category::Formatted, length = 255)]
            name: Option<Formatted>,
            #[msi_column(localizable, category = msi::Category::Formatted, length = 0)]
            value: Option<Formatted>,
            #[msi_column(identifier(foreign_key = "Component"), category = msi::Category::Identifier, length = 72)]
            component_: ComponentIdentifier,
        },


        /// Searches for file or directory using the registry.
        ///
        /// [*Reference*](https://learn.microsoft.com/en-us/windows/win32/msi/reglocator-table)
        RegLocator {
            #[msi_column(primary_key, identifier(foreign_key = "Signature"), category = msi::Category::Identifier, length = 72)]
            signature_: SignatureIdentifier,
            #[msi_column(category = msi::Category::Integer, length = 0)]
            root: RegistryRoot,
            #[msi_column(localizable, category = msi::Category::RegPath, length = 255)]
            key: RegPath,
            #[msi_column(localizable, category = msi::Category::Formatted, length = 255)]
            name: Option<Formatted>,
            #[msi_column(column_name = "Type", category = msi::Category::Integer)]
            typ: Option<LocatorType>,
        },

        // TODO: RemoveFile,
        // TODO: RemoveIniFile,
        // TODO: RemoveRegistry,
        // TODO: ReserveCost,
        // TODO: SelfReg,

        /// Controls installed or uninstalled services.
        ///
        /// ## Note
        /// Services that rely on the presence of an assembly in the Global Assembly Cache (GAC)
        /// cannot be installed or started using the ServiceInstall and ServiceControl tables. If
        /// you need to start a service that depends on an assembly in the GAC, you must use a
        /// custom action sequenced after the InstallFinalize action or a commit custom action. For
        /// information about installing assemblies to the GAC see Installation of Assemblies to
        /// the Global Assembly Cache.
        ///
        /// [*Reference*](https://learn.microsoft.com/en-us/windows/win32/msi/servicecontrol-table)
        ServiceControl {
            #[msi_column(primary_key, identifier(generated), category = msi::Category::Identifier, length = 72)]
            service_control: ServiceControlIdentifier,

            /// This column is the string naming the service. This column can be used to control a
            /// service that is not installed.
            #[msi_column(localizable, category = msi::Category::Formatted, length = 255)]
            name: Formatted,

            /// This column contains the operations to be performed on the named service. Note that
            /// when stopping a service, all services that depend on that service are also stopped.
            /// When deleting a service that is running, the installer stops the service.
            ///
            /// The values in this field are bit fields that can be combined into a single value
            /// that represents several operations.
            #[msi_column(category = msi::Category::Integer)]
            event: ServiceControlEvent,

            /// A list of arguments for starting services. The arguments are separated by null
            /// characters [~]. For example, the list of arguments One, Two, and Three are listed
            /// as: One[~]Two[~]Three.
            #[msi_column(localizable, category = msi::Category::Formatted, length = 255)]
            arguments: Option<Formatted>,

            /// Leaving this field null or entering a value of 1 causes the installer to wait a
            /// maximum of 30 seconds for the service to complete before proceeding. The wait can
            /// be used to allow additional time for a critical event to return a failure error. A
            /// value of 0 in this field means to wait only until the service control manager (SCM)
            /// reports that this service is in a pending state before continuing with the
            /// installation.
            #[msi_column(category = msi::Category::Integer)]
            wait: Option<Integer>,
            #[msi_column(identifier(foreign_key = "Component"), category = msi::Category::Identifier, length = 72)]
            component_: ComponentIdentifier,
        },

        /// Lists information used to install a service.
        ///
        /// [*Reference*](https://learn.microsoft.com/en-us/windows/win32/msi/serviceinstall-table)
        ServiceInstall {
            #[msi_column(primary_key, identifier(generated), category = msi::Category::Identifier, length = 72)]
            service_install: ServiceInstallIdentifier,
            #[msi_column(category = msi::Category::Formatted, length = 255)]
            name: Formatted,
            #[msi_column(category = msi::Category::Formatted, length = 255)]
            display_name: Option<Formatted>,
            #[msi_column(category = msi::Category::DoubleInteger)]
            service_type: ServiceType,
            #[msi_column(category = msi::Category::DoubleInteger)]
            start_type: StartType,
            #[msi_column(category = msi::Category::DoubleInteger)]
            error_control: ErrorControl,
            #[msi_column(category = msi::Category::Formatted, length = 255)]
            load_order_group: Option<Formatted>,
            #[msi_column(category = msi::Category::Formatted, length = 255)]
            dependencies: Option<Formatted>,
            #[msi_column(category = msi::Category::Formatted, length = 255)]
            start_name: Option<Formatted>,
            #[msi_column(category = msi::Category::Formatted, length = 255)]
            password: Option<Formatted>,
            #[msi_column(category = msi::Category::Formatted, length = 255)]
            arguments: Option<Formatted>,
            #[msi_column(identifier(foreign_key = "Component"), category = msi::Category::Identifier, length = 72)]
            component_: ComponentIdentifier,
            #[msi_column(localizable, category = msi::Category::Formatted, length = 255)]
            description: Option<Formatted>,
        },

        // TODO: SFPCatalog,

        /// Lists information needed to create shortcuts.
        ///
        /// The Shortcut table holds the information the application needs to create shortcuts on the user's computer.
        ///
        /// [*Reference*](https://learn.microsoft.com/en-us/windows/win32/msi/shortcut-table)
        Shortcut {
            #[msi_column(primary_key, identifier(generated), category = msi::Category::Identifier, length = 72)]
            shortcut: ShortcutIdentifier,
            #[msi_column(identifier(foreign_key = "Directory"), category = msi::Category::Identifier, length = 72)]
            directory_: DirectoryIdentifier,
            #[msi_column(localizable, category = msi::Category::Filename, length = 128)]
            name: Filename,
            #[msi_column(identifier(foreign_key = "Component"), category = msi::Category::Identifier, length = 72)]
            component_: ComponentIdentifier,
            #[msi_column(category = msi::Category::Shortcut, length = 72)]
            target: Shortcut,
            #[msi_column(category = msi::Category::Formatted, length = 255)]
            arguments: Option<Formatted>,
            #[msi_column(localizable, category = msi::Category::Formatted, length = 255)]
            description: Option<Text>,
            // TODO: Make this a bitflag. Has a lot of options so I'm going to punt it off to later.
            #[msi_column(category = msi::Category::Integer)]
            hotkey: Option<Integer>,
            #[msi_column(identifier(foreign_key = "Icon"), category = msi::Category::Identifier, length = 72)]
            icon_: Option<IconIdentifier>,
            #[msi_column(category = msi::Category::Integer)]
            icon_index: Option<Integer>,
            #[msi_column(category = msi::Category::Integer)]
            show_cmd: Option<ShowCmd>,
            // TODO: Determine if this is actually an `Identifier` type. The documentation for this field
            // says that the "Windows format [can be used] to reference environment variables, for example
            // %USERPROFILE%" but the `Identifier` type does not allow `%`.
            #[msi_column(category = msi::Category::Identifier, length = 72)]
            wk_dir: Option<Identifier>,
            #[msi_column(category = msi::Category::Formatted, length = 255)]
            display_resource_dll: Option<Formatted>,
            #[msi_column(category = msi::Category::Integer)]
            display_resource_id: Option<Integer>,
            #[msi_column(category = msi::Category::Formatted, length = 255)]
            description_resource_dll: Option<Formatted>,
            #[msi_column(category = msi::Category::Integer)]
            description_resource_id: Option<Integer>,
        },

        /// Lists the unique file signatures that identify files.
        ///
        /// The Signature table holds the information that uniquely identifies a file signature.
        /// For more information regarding signatures see Digital Signatures and Windows Installer.
        ///
        /// [*Reference*](https://learn.microsoft.com/en-us/windows/win32/msi/signature-table)
        Signature {
            #[msi_column(primary_key, identifier(generated), category = msi::Category::Identifier, length = 72)]
            signature: SignatureIdentifier,
            #[msi_column(category = msi::Category::Text, length = 255)]
            file_name: Text,
            #[msi_column(category = msi::Category::Text, length = 20)]
            min_version: Option<Text>,
            #[msi_column(category = msi::Category::Text, length = 20)]
            max_version: Option<Text>,
            #[msi_column(category = msi::Category::DoubleInteger)]
            min_size: Option<DoubleInteger>,
            #[msi_column(category = msi::Category::DoubleInteger)]
            max_size: Option<DoubleInteger>,
            #[msi_column(category = msi::Category::DoubleInteger)]
            min_date: Option<Date>,
            #[msi_column(category = msi::Category::DoubleInteger)]
            max_date: Option<Date>,
            #[msi_column(category = msi::Category::Text, length = 255)]
            languages: Option<Text>,
        },

        // TODO: TextStyle,
        // TODO: TypeLib,
        // TODO: UIText,
        // TODO: Verb,
        // TODO: Upgrade,
    }
}
