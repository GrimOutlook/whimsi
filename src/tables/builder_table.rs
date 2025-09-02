use std::io::{Read, Seek, Write};

use msi::Package;

pub(crate) trait MsiBuilderTable: Default {
    type TableValue;

    /// Utilized when creating the MSI using the `msi` crate.
    fn name(&self) -> &'static str;
    fn values(&self) -> &Vec<Self::TableValue>;
    fn columns(&self) -> Vec<msi::Column>;
    fn rows(&self) -> Vec<Vec<msi::Value>>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn add(&mut self, dao: Self::TableValue) -> anyhow::Result<()>;

    /// Write the columns contained in the table to the package.
    fn write_to_package<F: std::io::Read + std::io::Write + std::io::Seek>(
        &self,
        package: &mut msi::Package<F>,
    ) -> anyhow::Result<()> {
        let columns = self.columns();
        package.create_table(self.name(), columns)?;
        let query = msi::Insert::into(self.name()).rows(self.rows());
        Ok(package.insert_rows(query)?)
    }
}

#[macro_export]
macro_rules! msitable_boilerplate {
    () => {
        fn values(&self) -> &Vec<Self::TableValue> {
            &self.0
        }
        fn len(&self) -> usize {
            self.0.len()
        }
        fn is_empty(&self) -> bool {
            self.0.is_empty()
        }
    };
}
