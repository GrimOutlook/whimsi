use crate::tables::component::dao::ComponentDao;
use crate::tables::directory::dao::DirectoryDao;
use crate::tables::feature::dao::FeatureDao;
use crate::tables::feature_components::dao::FeatureComponentsDao;
use crate::tables::file::dao::FileDao;
use crate::tables::media::dao::MediaDao;
use crate::tables::msi_file_hash::dao::MsiFileHashDao;
use crate::tables::property::dao::PropertyDao;
use crate::tables::registry::dao::RegistryDao;

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
}

pub(crate) trait IsDao {
    fn to_row(&self) -> Vec<msi::Value>;
}
