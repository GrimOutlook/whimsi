use super::directory::dao::DirectoryDao;
use super::file::dao::FileDao;
use crate::types::column::identifier::Identifier;

type ComponentIdentifier = Identifier;

#[derive(Clone, Debug)]
pub enum TableEntry {
    Directory(DirectoryDao),
    File((FileDao, ComponentIdentifier)),
}
