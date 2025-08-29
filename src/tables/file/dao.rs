use derive_more::Constructor;
use getset::Getters;
use msi::Language;

use crate::types::{
    column::{
        attributes::Attributes, identifier::Identifier, sequence::Sequence, version::Version,
    },
    helpers::filename::Filename,
};

use super::helper::File;

#[derive(Clone, Debug, PartialEq, Getters, Constructor)]
#[getset(get = "pub")]
pub struct FileDao {
    file: Identifier,
    Component_: Identifier,
    name: Filename,
    size: u32,
    version: Option<Version>,
    language: Option<Language>,
    attributes: Option<Attributes>,
    sequence: Sequence,
}

impl FileDao {
    pub(crate) fn from_file(
        file: &File,
        id: &Identifier,
        component_id: &Identifier,
        sequence: Sequence,
    ) -> anyhow::Result<FileDao> {
        todo!("from_file")
    }
}
