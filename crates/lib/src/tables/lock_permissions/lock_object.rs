use crate::tables::file::table::FileIdentifier;
use crate::tables::registry::dao::RegistryIdentifier;
use crate::tables::service_install::table::ServiceInstallIdentifier;
use crate::types::column::identifier::Identifier;
use crate::types::column::identifier::ToIdentifier;
use crate::types::column::identifier::ambassador_impl_ToIdentifier;

#[derive(
    Debug, Clone, PartialEq, ambassador::Delegate, strum::IntoStaticStr,
)]
#[delegate(ToIdentifier)]
pub enum LockObject {
    File(FileIdentifier),
    Registry(RegistryIdentifier),
    // CreateFolder(CreateFolderIdentifier),
    ServiceInstall(ServiceInstallIdentifier),
}

impl LockObject {
    pub fn table(&self) -> &'static str {
        self.into()
    }
}
