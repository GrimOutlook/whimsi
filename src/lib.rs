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
// #![cfg(not(debug_assertions))]
// #![deny(
//     clippy::all,
//     missing_docs,
//     missing_debug_implementations,
//     rustdoc::all,
//     unsafe_code
// )]
#![cfg(debug_assertions)]
#![allow(dead_code)]

mod tables;

use tables::{
    component::Component, directory::Directory, environment_variable::EnvironmentVariable,
    feature::Feature, file::File, permission::Permission, registry::Registry, service::Service,
    shortcut::Shortcut,
};

/// An in-memory representation of the final MSI to be created.
///
/// Properties are derived from this table:
/// https://learn.microsoft.com/en-us/windows/win32/msi/database-tables
///
/// WARN: This is missing many possible tables as seen when checking the above resource. I have
/// only implemented the tables that I believe will be useful for my usecases at this moment.
pub struct Msi {
    /// Lists installation components.
    ///
    /// Table Information Contained:
    /// - [`Component`](https://learn.microsoft.com/en-us/windows/win32/msi/component-table)
    components: Vec<Component>,
    /// Directory layout for the application.
    ///
    /// Table Information Contained:
    /// - ['Directory'](https://learn.microsoft.com/en-us/windows/win32/msi/directory-table)
    directories: Vec<Directory>,
    /// Complete list of source files with their attributes.
    ///
    /// Table Information Contained:
    /// - ['File'](https://learn.microsoft.com/en-us/windows/win32/msi/file-table)
    files: Vec<File>,
    /// Defines the logical tree structure of features.
    ///
    /// Table Information Contained:
    /// - ['Feature'](https://learn.microsoft.com/en-us/windows/win32/msi/feature-table)
    /// - ['FeatureComponents'](https://learn.microsoft.com/en-us/windows/win32/msi/featurecomponents-table)
    ///
    /// NOTE: The [feature-components
    /// table](https://learn.microsoft.com/en-us/windows/win32/msi/featurecomponents-table) is not used.
    /// Instead features have an extra property which is a vec holding references to the components that they contain.
    features: Vec<Feature>,
    /// Lists information needed to create shortcuts.
    ///
    /// Table Information Contained:
    /// - ['Shortcut'](https://learn.microsoft.com/en-us/windows/win32/msi/shortcut-table)
    shortcuts: Vec<Shortcut>,
    /// Secures services, files, registry keys, and created folders
    ///
    /// Table Information Contained:
    /// - ['MsiLockPermissionsEx'](https://learn.microsoft.com/en-us/windows/win32/msi/msilockpermissionsex-table)
    permissions: Vec<Permission>,
    /// Lists information used to install a service.
    ///
    /// Table Information Contained:
    /// - ['MsiServiceConfig'](https://learn.microsoft.com/en-us/windows/win32/msi/msiserviceconfig-table)
    /// - ['ServiceInstall'](https://learn.microsoft.com/en-us/windows/win32/msi/serviceinstall-table)
    /// - ['ServiceControl'](https://learn.microsoft.com/en-us/windows/win32/msi/serviceinstall-table)
    services: Vec<Service>,
    /// Lists registry information for the application.
    ///
    /// Table Information Contained:
    /// - ['Resgistry'](https://learn.microsoft.com/en-us/windows/win32/msi/registry-table)
    registry_entries: Vec<Registry>,

    /// Lists the environment variables.
    ///
    /// Table Information Contained:
    /// - ['Environment'](https://learn.microsoft.com/en-us/windows/win32/msi/environment-table)
    environment_varaiables: Vec<EnvironmentVariable>,
}
