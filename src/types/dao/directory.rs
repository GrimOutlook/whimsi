use crate::types::{
    column::{default_dir::DefaultDir, identifier::Identifier},
    helpers::directory::{DirectoryKind, RootDirectory},
};

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct DirectoryDao {
    default: DefaultDir,
    directory: Identifier,
    parent: Identifier,
}

impl DirectoryDao {
    pub fn new(
        directory: &impl DirectoryKind,
        parent: &impl DirectoryKind,
    ) -> anyhow::Result<DirectoryDao> {
        todo!()
    }
}

impl From<&RootDirectory> for DirectoryDao {
    fn from(value: &RootDirectory) -> Self {
        // The documentation says that only the root directory can have a `directory` and `parent`
        // field containing the same value. Figured it would be simpler to do this than make the
        // `parent` field optional and have to unwrap it for every operation not involving the root
        // directory.
        Self {
            directory: value.id().clone().into(),
            parent: value.id().clone().into(),
            default: value.name().clone().into(),
        }
    }
}
