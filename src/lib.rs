pub mod types;

// This is basically an internal representation of what is contained in the final MSI, just in more
// manipulatable data types.
//
// Properties are derived from this table:
// https://learn.microsoft.com/en-us/windows/win32/msi/database-tables
pub struct Msi {
    /// Lists installation components.
    components: Vec<Component>,
    /// Directory layout for the application.
    directories: Vec<Directory>,
    /// Complete list of source files with their attributes.
    files: Vec<File>,
    /// Defines the logical tree structure of features.
    //
    // NOTE: The [feature-components
    // table](https://learn.microsoft.com/en-us/windows/win32/msi/featurecomponents-table) is not used.
    // Instead features hold references to the components that they contain.
    features: Vec<Feature>,
    ///  	Lists information needed to create shortcuts.
    shortcuts: Vec<Shortcut>,
    /// Secures services, files, registry keys, and created folders
    permissions: Vec<MsiLockPermissionsEx>,
    /// Lists information used to install a service.
    ///
    /// NOTE: Real name for the table is ServiceInstall but idc.
    /// NOTE: Also encompasses the MsiServiceConfig table.
    services: Vec<Service>,
    /// Lists registry information for the application.
    ///
    /// NOTE: This also encompasses the RemoveRegistry table. `Registry` is an enum with 2 types.
    /// Remove and Add.
    registry_entries: Vec<Registry>,
}

struct Component;
struct Directory;
struct File;
struct Feature;
struct MsiLockPermissionsEx;
struct Shortcut;
struct Service;
struct Registry;
