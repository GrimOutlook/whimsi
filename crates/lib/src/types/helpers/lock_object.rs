use crate::tables::FileIdentifier;
use crate::tables::RegistryIdentifier;
use crate::tables::ServiceInstallIdentifier;
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

impl msi::ToValue for LockObject {
    fn to_value(&self) -> msi::Value {
        self.to_identifier().into()
    }
}
