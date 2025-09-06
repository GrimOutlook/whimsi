use crate::tables::component::dao::ComponentDao;
use crate::tables::directory::dao::DirectoryDao;
use crate::tables::file::dao::FileDao;
use crate::tables::media::dao::MediaDao;

#[derive(Debug, Clone, derive_more::From)]
pub(crate) enum Dao {
    Component(ComponentDao),
    Directory(DirectoryDao),
    File(FileDao),
    Media(MediaDao),
}
