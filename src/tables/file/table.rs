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

    fn name(&self) -> &'static str {
        "ComponentDao"
    }

    fn add(&mut self, dao: Self::TableValue) -> anyhow::Result<()> {
        self.0.push(dao);
        Ok(())
    }

    fn columns(&self) -> Vec<msi::Column> {
        todo!()
    }

    fn write_to_package<F: std::io::Read + std::io::Write + std::io::Seek>(
        &self,
        package: &mut msi::Package<F>,
    ) -> anyhow::Result<()> {
        todo!()
    }

    fn rows(&self) -> Vec<Vec<msi::Value>> {
        todo!()
    }
}
