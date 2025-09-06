use anyhow::ensure;
use itertools::Itertools;

use super::dao::FileDao;
use crate::constants::*;
use crate::msitable_boilerplate;
use crate::tables::builder_table::MsiBuilderTable;
use crate::types::column::identifier::Identifier;

#[derive(Debug, Clone, Default)]
pub struct FileTable(Vec<FileDao>);
impl MsiBuilderTable for FileTable {
    type TableValue = FileDao;

    // Boilderplate needed to access information on the inner object
    msitable_boilerplate!();

    fn name(&self) -> &'static str {
        "File"
    }

    fn columns(&self) -> Vec<msi::Column> {
        vec![
            msi::Column::build("File")
                .primary_key()
                .id_string(IDENTIFIER_MAX_LEN),
            msi::Column::build("Component_").id_string(IDENTIFIER_MAX_LEN),
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
