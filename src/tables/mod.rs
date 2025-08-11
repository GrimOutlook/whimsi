use directory::DirectoryTable;
use strum::EnumDiscriminants;

pub mod directory;

pub trait MsiTable {
    type TableValue;

    fn name() -> &'static str;
    fn init() -> Self;
    fn default_values() -> Vec<Self::TableValue>;
    fn values(&self) -> Vec<Self::TableValue>;
}

/// Enum values are derived from this table:
/// https://learn.microsoft.com/en-us/windows/win32/msi/database-tables
///
/// WARN: This is missing many possible tables as seen when checking the above resource. I have
/// only implemented the tables that I believe will be useful for my usecases at this moment.
///
#[derive(EnumDiscriminants)]
#[strum_discriminants(name(TableKind))]
pub enum Table {
    /// Directory layout for the application.
    ///
    /// Table Information Contained:
    /// - ['Directory'](https://learn.microsoft.com/en-us/windows/win32/msi/directory-table)
    Directories(DirectoryTable),
    // /// Complete list of source files with their attributes.
    // ///
    // /// Table Information Contained:
    // /// - ['File'](https://learn.microsoft.com/en-us/windows/win32/msi/file-table)
    // Files(Files),
    //
    // /// Lists installation components.
    // ///
    // /// Table Information Contained:
    // /// - [`Component`](https://learn.microsoft.com/en-us/windows/win32/msi/component-table)
    // Components(Components),
    //
    // /// Defines the logical tree structure of features.
    // ///
    // /// Table Information Contained:
    // /// - ['Feature'](https://learn.microsoft.com/en-us/windows/win32/msi/feature-table)
    // /// - ['FeatureComponents'](https://learn.microsoft.com/en-us/windows/win32/msi/featurecomponents-table)
    // ///
    // /// NOTE: The [feature-components
    // /// table](https://learn.microsoft.com/en-us/windows/win32/msi/featurecomponents-table) is not used.
    // /// Instead features have an extra property which is a vec holding references to the components that they contain.
    // Features(Features),
    //
    // /// Lists information needed to create shortcuts.
    // ///
    // /// Table Information Contained:
    // /// - ['Shortcut'](https://learn.microsoft.com/en-us/windows/win32/msi/shortcut-table)
    // Shortcuts(Shortcuts),
    //
    // /// Secures services, files, registry keys, and created folders
    // ///
    // /// Table Information Contained:
    // /// - ['MsiLockPermissionsEx'](https://learn.microsoft.com/en-us/windows/win32/msi/msilockpermissionsex-table)
    // Permissions(Permissions),
    //
    // /// Lists information used to install a service.
    // ///
    // /// Table Information Contained:
    // /// - ['MsiServiceConfig'](https://learn.microsoft.com/en-us/windows/win32/msi/msiserviceconfig-table)
    // /// - ['ServiceInstall'](https://learn.microsoft.com/en-us/windows/win32/msi/serviceinstall-table)
    // /// - ['ServiceControl'](https://learn.microsoft.com/en-us/windows/win32/msi/serviceinstall-table)
    // Services(Services),
    //
    // /// Lists registry information for the application.
    // ///
    // /// Table Information Contained:
    // /// - ['Registry'](https://learn.microsoft.com/en-us/windows/win32/msi/registry-table)
    // RegistryEntries(RegistryEntries),
    //
    // /// Lists the environment variables.
    // ///
    // /// Table Information Contained:
    // /// - ['Environment'](https://learn.microsoft.com/en-us/windows/win32/msi/environment-table)
    // EnvironmentVariables(EnvironmentVariables),
}
