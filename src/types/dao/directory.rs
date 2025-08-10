use crate::types::{
    column::{default_dir::DefaultDir, identifier::Identifier},
    helpers::directory::{Directory, RootDirectory},
};

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct DirectoryDao {
    default: DefaultDir,
    directory: Identifier,
    parent: Identifier,
}

impl DirectoryDao {
    pub fn new(directory: &Directory, parent: &Directory) -> anyhow::Result<DirectoryDao> {
        todo!()
    }
}

impl From<RootDirectory> for DirectoryDao {
    fn from(value: RootDirectory) -> Self {
        Self {
            directory: value.id().clone(),
            parent: value.id().clone(),
            default: value.name().clone().into(),
        }
    }
}
