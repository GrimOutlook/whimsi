pub mod admin_execute_sequence;
pub mod admin_ui_sequence;
pub mod advt_execute_sequence;
pub mod app_search;
pub mod binary;
pub mod builder_list;
pub mod builder_list_entry;
pub(crate) mod builder_table;
pub mod component;
pub mod custom_action;
pub(crate) mod dao;
pub mod directory;
pub mod feature;
pub mod feature_components;
pub mod file;
pub mod generic_sequence;
pub(crate) mod id_generator_builder_list;
pub mod install_execute_sequence;
pub mod install_ui_sequence;
pub mod launch_condition;
pub(crate) mod macros;
pub mod media;
pub mod meta;
pub mod msi_file_hash;
pub mod property;
pub mod reg_locator;
pub mod registry;
pub mod signature;
pub mod table_entry;

// TODO: Look at Directory to see the form that I eventually want to have implemented.
#[derive(strum::EnumIter)]
enum Table {
    ActionTable,
    AdminExecuteSequence,
    AdminUiSequence,
    AdvtExecuteSquence,
    AdvtUiSequence,
    AppId,
    AppSearch,
    BBControl,
    Billboard,
    Binary,
    BindImage,
    CCPSearch,
    CheckBox,
    Class,
    CoimboBox,
    CompLocators,
    Complus,
    Component,
    Condition,
    Control,
    ControlCondition,
    ControlEvent,
    CreateFolder,
    CustomAction,
    Dialog,

    /// Directory layout for the application
    ///
    /// [*Reference*](https://learn.microsoft.com/en-us/windows/win32/msi/directory-table)
    ///
    // TODO: Implement the macros below as these are how I want to define the tables.
    //
    // // The MsiTable macro creates all of the boilerplate functions that depend on the name of
    // // the table. The `#` next to `DirectoryIdentifier` indicates that this field is the primary
    // // key in the table.
    // //
    // // For tables with identifiers that need to be generated before the MSI is created, the
    // // IdentifierGenerator macro will create a [TABLE]Identifier struct that implments the
    // // ToIdentifier trait.
    // #[derive(MsiTable, IdentifierGenerator)]
    // #[msi_table(
    //     DefaultDir: Value::Str,
    //     #DirectoryIdentifier: Value::Str,
    //     Option<DirectoryIdentifier>: Value::Str
    // )]
    Directory,
    DrLocator,
    DuplicateFile,
    Environment,
    Error,
    EventMapping,
    Extensiuon,
    Feature,
    FeatureComponents,
    File,
    FileSFPCatqalog,
    Font,
    Icon,
    IniFile,
    IniLocator,
    InstallExecuteSequence,
    InstallUISequence,
    IsolatedComponent,
    LaunchCondition,
    ListBox,
    ListView,
    LockPermissions,
    Media,
    MIME,
    MoveFile,
    MsiAssembly,
    MsiAssemblyName,
    MsiDigitalCertificate,
    MsiDigitalSiganture,
    MsiEmbeddedChainer,
    MsiEmbeddedUI,
    MsiFileHash,
    MsiLockFilePermissionsEx,
    MsiPackageCertificate,
    MsiPatchCertificate,
    MsiPatchHeaders,
    MsiPatchMetadata,
    MsiPatchOldAssemblyName,
    MsiPatchOldAssemblyFile,
    MsiPatchSequence,
    MsiServiceConfig,
    MsiServiceConfigFailureActions,
    MsiSFFCBypass,
    ODBCAttribute,
    ODBCDataSource,
    ODBCDrive,
    ODBCSourceAttribute,
    ODBCTranslator,
    Patch,
    PatchPackage,
    ProgId,
    Property,
    PublishComponent,
    RadioButton,
    Reguistry,
    RegLocator,
    RemoveFile,
    RemoveIniFile,
    RemoveRegistry,
    ReserveCost,
    SelfReg,
    ServiceControl,
    ServiceInstall,
    SFPCatalog,
    Shortcut,
    Signature,
    TextStyle,
    TypeLib,
    UIText,
    Verb,
    Upgrade,
}
