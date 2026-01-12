use crate::tables::file::table::FileIdentifier;
use crate::tables::registry::dao::RegistryIdentifier;

#[derive(
    Clone,
    Debug,
    PartialEq,
    derive_more::Display,
    derive_more::From,
    whimsi_macros::IntoStrMsiValue,
)]
pub enum KeyPath {
    File(FileIdentifier),
    Registry(RegistryIdentifier),
    // ODBCDataSource(ODBCDataSourceIdentifier)
}
