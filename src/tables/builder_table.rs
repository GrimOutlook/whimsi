use std::io::{Read, Seek, Write};

use msi::Package;
use tracing::{debug, trace};

pub(crate) trait MsiBuilderTable: Default {
    type TableValue;

    /// Utilized when creating the MSI using the `msi` crate.
    fn name(&self) -> &'static str;
    fn values(&self) -> &Vec<Self::TableValue>;
    fn columns(&self) -> Vec<msi::Column>;
    fn rows(&self) -> Vec<Vec<msi::Value>>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn contains(&self, dao: &Self::TableValue) -> bool;
    fn add(&mut self, dao: Self::TableValue) -> anyhow::Result<()>;
    fn add_all(&mut self, daos: Vec<Self::TableValue>) -> anyhow::Result<()> {
        daos.into_iter()
            .map(|dao| self.add(dao))
            .collect::<anyhow::Result<Vec<()>>>()?;
        Ok(())
    }

    /// Write the columns contained in the table to the package.
    fn write_to_package<F: std::io::Read + std::io::Write + std::io::Seek>(
        &self,
        package: &mut msi::Package<F>,
    ) -> anyhow::Result<()> {
        debug!("Writing {}Table to package", self.name());
        let columns = self.columns();
        package.create_table(self.name(), columns)?;
        let rows = self.rows();
        trace!("Inserting rows into {}Table:", self.name());
        rows.clone()
            .iter()
            .enumerate()
            .for_each(|(index, r)| trace!("{index}: {r:?}"));
        let query = msi::Insert::into(self.name()).rows(rows);
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
