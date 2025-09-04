use derive_more::Constructor;
use getset::Getters;
use msi::Language;

use crate::dint_val;
use crate::int_val;
use crate::opt_str_val;
use crate::opt_val;
use crate::str_val;
use crate::types::column::attributes::Attributes;
use crate::types::column::identifier::Identifier;
use crate::types::column::sequence::Sequence;
use crate::types::column::version::Version;
use crate::types::helpers::filename::Filename;

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
    pub fn to_row(&self) -> Vec<msi::Value> {
        vec![
            str_val!(self.file),
            str_val!(self.component),
            str_val!(self.name),
            dint_val!(self.size),
            opt_str_val!(self.version),
            opt_val!(self.language),
            opt_val!(self.attributes),
            int_val!(Into::<i16>::into(self.sequence.clone())),
        ]
    }
}
