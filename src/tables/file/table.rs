use anyhow::ensure;
use itertools::Itertools;

use crate::{
    constants::*, msitable_boilerplate, tables::builder_table::MsiBuilderTable,
    types::column::identifier::Identifier,
};

use super::dao::FileDao;

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

    fn rows(&self) -> Vec<Vec<msi::Value>> {
        self.values().iter().map(FileDao::to_row).collect_vec()
    }

    fn contains(&self, dao: &FileDao) -> bool {
        self.0
            .iter()
            .find(|entry| entry.file() == dao.file())
            .is_some()
    }

    fn add(&mut self, dao: Self::TableValue) -> anyhow::Result<()> {
        // TODO: Create actual error for file ID collision.
        ensure!(!self.contains(&dao), "TEMPERROR");
        self.0.push(dao);
        Ok(())
    }
}
