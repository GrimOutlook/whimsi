pub mod builder_list;
pub(crate) mod builder_table;
pub(crate) mod dao;
pub(crate) mod id_generator_builder_list;
pub mod meta;

use crate as whimsi_lib;
use crate::tables::builder_table::MsiTable;
use crate::tables::dao::MsiDao;
use crate::types::column::default_dir::DefaultDir;
use crate::types::helpers::primary_identifier::PrimaryIdentifier;

// TODO: Look at Directory to see the form that I eventually want to have implemented.

#[derive(strum::EnumIter, whimsi_table_macro::MsiTables)]
enum TableVariants {
    // ActionTable,
    // AdminExecuteSequence,
    // AdminUiSequence,
    // AdvtExecuteSquence,
    // AdvtUiSequence,
    // AppId,
    // AppSearch,
    // BBControl,
    // Billboard,
    // Binary,
    // BindImage,
    // CCPSearch,
    // CheckBox,
    // Class,
    // CoimboBox,
    // CompLocators,
    // Complus,
    // Component,
    // Condition,
    // Control,
    // ControlCondition,
    // ControlEvent,
    // CreateFolder,
    // CustomAction,
    // Dialog,
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
    Directory {
        #[msitable(primary_key, identifier(generated), category = msi::Category::Identifier, length = 72)]
        directory: DirectoryIdentifier,
        #[msitable(identifier(foreign_key = "Directory"), column_name = "Directory_Parent", category = msi::Category::Identifier, length = 72)]
        parent_directory: Option<DirectoryIdentifier>,
        #[msitable(localizable, category = msi::Category::DefaultDir, length = 255)]
        default_dir: DefaultDir,
    },
    // DrLocator,
    // DuplicateFile,
    // Environment,
    // Error,
    // EventMapping,
    // Extensiuon,
    // Feature,
    // FeatureComponents,
    // File,
    // FileSFPCatqalog,
    // Font,
    // Icon,
    // IniFile,
    // IniLocator,
    // InstallExecuteSequence,
    // InstallUISequence,
    // IsolatedComponent,
    // LaunchCondition,
    // ListBox,
    // ListView,
    // LockPermissions,
    // Media,
    // MIME,
    // MoveFile,
    // MsiAssembly,
    // MsiAssemblyName,
    // MsiDigitalCertificate,
    // MsiDigitalSiganture,
    // MsiEmbeddedChainer,
    // MsiEmbeddedUI,
    // MsiFileHash,
    // MsiLockFilePermissionsEx,
    // MsiPackageCertificate,
    // MsiPatchCertificate,
    // MsiPatchHeaders,
    // MsiPatchMetadata,
    // MsiPatchOldAssemblyName,
    // MsiPatchOldAssemblyFile,
    // MsiPatchSequence,
    // MsiServiceConfig,
    // MsiServiceConfigFailureActions,
    // MsiSFFCBypass,
    // ODBCAttribute,
    // ODBCDataSource,
    // ODBCDrive,
    // ODBCSourceAttribute,
    // ODBCTranslator,
    // Patch,
    // PatchPackage,
    // ProgId,
    // Property,
    // PublishComponent,
    // RadioButton,
    // Reguistry,
    // RegLocator,
    // RemoveFile,
    // RemoveIniFile,
    // RemoveRegistry,
    // ReserveCost,
    // SelfReg,
    // ServiceControl,
    // ServiceInstall,
    // SFPCatalog,
    // Shortcut,
    // Signature,
    // TextStyle,
    // TypeLib,
    // UIText,
    // Verb,
    // Upgrade,
}
