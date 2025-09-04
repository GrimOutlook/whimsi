use super::{helper::Directory, system_directory::SystemDirectory};

#[derive(Debug, Clone, derive_more::From)]
pub(crate) enum DirectoryKind {
    Directory(Directory),
    SystemDirectory(SystemDirectory),
}
