use crate::types::column::identifier::Identifier;

use super::{directory::dao::DirectoryDao, file::dao::FileDao};

type ComponentIdentifier = Identifier;

#[derive(Clone, Debug)]
pub enum TableEntry {
    Directory(DirectoryDao),
    File((FileDao, ComponentIdentifier)),
}
