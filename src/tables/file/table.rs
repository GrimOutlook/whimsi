use crate::{
    msitable_boilerplate, tables::builder_table::MsiBuilderTable,
    types::column::identifier::Identifier,
};

use super::dao::FileDao;

#[derive(Debug, Clone, Default)]
pub struct FileTable(Vec<FileDao>);
impl MsiBuilderTable for FileTable {
    type TableValue = FileDao;

    // Boilderplate needed to access information on the inner object
    msitable_boilerplate!();

    fn name() -> &'static str {
        "ComponentDao"
    }

    fn default_values() -> Vec<Self::TableValue> {
        todo!()
    }
    fn add(&mut self, dao: Self::TableValue) -> anyhow::Result<()> {
        self.0.push(dao);
        Ok(())
    }
}
