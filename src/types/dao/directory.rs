use crate::types::{
    column::{default_dir::DefaultDir, identifier::Identifier},
    helpers::directory::Directory,
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
