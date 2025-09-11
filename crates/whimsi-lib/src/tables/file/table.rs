use anyhow::ensure;
use itertools::Itertools;

use super::dao::FileDao;
use crate::constants::*;
use crate::define_generator_table;
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

define_specific_identifier!(File);
define_specific_identifier_parsing!(File);
define_identifier_generator!(File);

define_generator_table!(
    File,
    vec![
        whimsi_msi::Column::build("File")
            .primary_key()
            .id_string(DEFAULT_IDENTIFIER_MAX_LEN),
        whimsi_msi::Column::build("Component_").id_string(DEFAULT_IDENTIFIER_MAX_LEN),
        whimsi_msi::Column::build("FileName")
            .category(whimsi_msi::Category::Filename)
            .string(FILENAME_MAX_LEN),
        whimsi_msi::Column::build("FileSize").int32(),
        whimsi_msi::Column::build("Version")
            .nullable()
            .category(whimsi_msi::Category::Version)
            .string(VERSION_MAX_LEN),
        whimsi_msi::Column::build("Language")
            .nullable()
            .category(whimsi_msi::Category::Language)
            .string(LANGUAGE_MAX_LEN),
        whimsi_msi::Column::build("Attributes").nullable().int16(),
        whimsi_msi::Column::build("Sequence").int16(),
    ]
);

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

msi_list_boilerplate!(FileTable, FileDao);
implement_new_for_id_generator_table!(FileTable, FileIdGenerator);
implement_id_generator_for_table!(FileTable, FileIdGenerator);
