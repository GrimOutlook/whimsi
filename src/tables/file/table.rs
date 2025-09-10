use anyhow::ensure;
use itertools::Itertools;

use super::dao::FileDao;
use crate::constants::*;
use crate::define_identifier_generator;
use crate::define_specific_identifier;
use crate::define_specific_identifier_parsing;
use crate::implement_id_generator_for_table;
use crate::implement_new_for_id_generator_table;
use crate::msi_list_boilerplate;
use crate::msi_table_boilerplate;
use crate::tables::builder_list::MsiBuilderList;
use crate::tables::builder_table::MsiBuilderTable;
use crate::types::column::identifier::Identifier;
use crate::types::column::sequence::Sequence;

define_specific_identifier!(file);
define_specific_identifier_parsing!(file);
define_identifier_generator!(file);

#[derive(Debug, Clone)]
pub struct FileTable {
    entries: Vec<FileDao>,
    generator: FileIdGenerator,
}

impl FileTable {
    pub fn in_sequence_range(&self, min: i16, max: i16) -> Vec<&FileDao> {
        self.entries
            .iter()
            .filter(|file| {
                if let Sequence::Included(sequence) = file.sequence() {
                    let sequence = sequence.to_i16();
                    return sequence >= min && sequence <= max;
                }
                false
            })
            .collect()
    }
}
impl MsiBuilderTable for FileTable {
    type TableValue = FileDao;

    // Boilderplate needed to access information on the inner object
    msi_table_boilerplate!();

    fn name(&self) -> &'static str {
        "File"
    }

    fn columns(&self) -> Vec<msi::Column> {
        vec![
            msi::Column::build("File")
                .primary_key()
                .id_string(DEFAULT_IDENTIFIER_MAX_LEN),
            msi::Column::build("Component_")
                .id_string(DEFAULT_IDENTIFIER_MAX_LEN),
            msi::Column::build("FileName")
                .category(msi::Category::Filename)
                .string(FILENAME_MAX_LEN),
            msi::Column::build("FileSize").int32(),
            msi::Column::build("Version")
                .nullable()
                .category(msi::Category::Version)
                .string(VERSION_MAX_LEN),
            msi::Column::build("Language")
                .nullable()
                .category(msi::Category::Language)
                .string(LANGUAGE_MAX_LEN),
            msi::Column::build("Attributes").nullable().int16(),
            msi::Column::build("Sequence").int16(),
        ]
    }
}

impl MsiBuilderList for FileTable {
    type ListValue = FileDao;

    msi_list_boilerplate!();
}

implement_new_for_id_generator_table!(FileTable, FileIdGenerator);
implement_id_generator_for_table!(FileTable, FileIdGenerator);
