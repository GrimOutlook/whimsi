use crate::tables::{FileIdentifier, RegistryIdentifier};
use crate::types::column::identifier::{
    Identifier, ToIdentifier, ambassador_impl_ToIdentifier,
};

/// Valid values found [here](https://learn.microsoft.com/en-us/windows/win32/msi/component-table#KeyPath)
#[derive(
    Clone,
    Debug,
    PartialEq,
    ambassador::Delegate,
    whimsi_macros::IdentifierToValue,
)]
#[delegate(ToIdentifier)]
pub enum KeyPath {
    File(FileIdentifier),
    Registry(RegistryIdentifier),
    // ODBCDataSource(ODBCDataSourceIdentifier)
}
