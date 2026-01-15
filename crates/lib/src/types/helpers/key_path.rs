use crate::types::column::identifier::Identifier;
use crate::types::column::identifier::IntoIdentifier;
use crate::types::column::identifier::ambassador_impl_IntoIdentifier;
use crate::types::helpers::to_msi_value::IntoMsiValue;

/// Valid values found [here](https://learn.microsoft.com/en-us/windows/win32/msi/component-table#KeyPath)
#[derive(
    Clone,
    Debug,
    PartialEq,
    ambassador::Delegate,
    whimsi_macros::IdentifierToValue,
)]
#[delegate(IntoIdentifier)]
pub enum KeyPath {
    File(Identifier),
    Registry(Identifier),
    ODBCDataSource(Identifier),
}
