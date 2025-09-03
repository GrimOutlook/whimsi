use derive_more::Constructor;
use getset::Getters;
use msi::Language;

use crate::{
    dint_val, int_val, opt_str_val, opt_val, str_val,
    types::{
        column::{
            attributes::Attributes, identifier::Identifier, sequence::Sequence, version::Version,
        },
        helpers::filename::Filename,
    },
};

use super::helper::File;

#[derive(Clone, Debug, PartialEq, Getters, Constructor)]
#[getset(get = "pub")]
pub struct FileDao {
    file: Identifier,
    component: Identifier,
    name: Filename,
    size: i32,
    version: Option<Version>,
    language: Option<Language>,
    attributes: Option<i16>,
    sequence: Sequence,
}

impl FileDao {
    pub(crate) fn from_file(
        file: &File,
        id: &Identifier,
        sequence: Sequence,
    ) -> anyhow::Result<FileDao> {
        todo!("from_file")
    }

    pub fn to_row(&self) -> Vec<msi::Value> {
        vec![
            str_val!(self.file),
            str_val!(self.component),
            str_val!(self.name),
            dint_val!(self.size),
            opt_str_val!(self.version),
            opt_val!(self.language),
            opt_val!(self.attributes),
            int_val!(Into::<i16>::into(&self.sequence)),
        ]
    }
}
