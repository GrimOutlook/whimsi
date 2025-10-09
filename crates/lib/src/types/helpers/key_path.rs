use crate::tables::{FileIdentifier, RegistryIdentifier};
use crate::types::column::identifier::{
    Identifier, ToIdentifier, ambassador_impl_ToIdentifier,
};

/// Valid values found [here](https://learn.microsoft.com/en-us/windows/win32/msi/component-table#KeyPath)
#[derive(Clone, Debug, PartialEq, ambassador::Delegate)]
#[delegate(ToIdentifier)]
pub enum KeyPath {
    File(FileIdentifier),
    Registry(RegistryIdentifier),
    // ODBCDataSource(ODBCDataSourceIdentifier)
}

impl msi::ToValue for KeyPath {
    fn to_value(&self) -> msi::Value {
        self.to_identifier().into()
    }
}
