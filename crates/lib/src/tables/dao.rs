use crate::tables::component::dao::ComponentDao;
use crate::tables::directory::dao::DirectoryDao;
use crate::tables::feature::dao::FeatureDao;
use crate::tables::feature_components::dao::FeatureComponentsDao;
use crate::tables::file::dao::FileDao;
use crate::tables::lock_permissions::dao::LockPermissionsDao;
use crate::tables::media::dao::MediaDao;
use crate::tables::msi_file_hash::dao::MsiFileHashDao;
use crate::tables::property::dao::PropertyDao;
use crate::tables::registry::dao::RegistryDao;
use crate::tables::service_control::dao::ServiceControlDao;
use crate::tables::service_install::dao::ServiceInstallDao;
use crate::tables::shortcut::dao::ShortcutDao;

#[derive(Debug, Clone, derive_more::From)]
pub(crate) enum Dao {
    Component(ComponentDao),
    Directory(DirectoryDao),
    File(FileDao),
    Media(MediaDao),
    Property(PropertyDao),
    MsiFileHash(MsiFileHashDao),
    Registry(RegistryDao),
    Feature(FeatureDao),
    FeatureComponents(FeatureComponentsDao),
    Shortcut(ShortcutDao),
    ServiceInstall(ServiceInstallDao),
    ServiceControl(ServiceControlDao),
    LockPermissions(LockPermissionsDao),
}

pub(crate) trait IsDao {
    fn to_row(&self) -> Vec<whimsi_msi::Value>;
}
