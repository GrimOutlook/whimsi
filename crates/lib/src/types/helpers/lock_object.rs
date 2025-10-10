use crate::tables::FileIdentifier;
use crate::tables::RegistryIdentifier;
use crate::tables::ServiceInstallIdentifier;
use crate::types::column::identifier::Identifier;
use crate::types::column::identifier::ToIdentifier;
use crate::types::column::identifier::ambassador_impl_ToIdentifier;

#[derive(
    Debug,
    Clone,
    PartialEq,
    ambassador::Delegate,
    strum::Display,
    whimsi_macros::IdentifierToValue,
)]
#[delegate(ToIdentifier)]
pub enum LockObject {
    File(FileIdentifier),
    Registry(RegistryIdentifier),
    // CreateFolder(CreateFolderIdentifier),
    ServiceInstall(ServiceInstallIdentifier),
}

impl LockObject {
    pub fn table(&self) -> String {
        self.to_string()
    }
}
