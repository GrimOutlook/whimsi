use std::io::Read;
use std::io::Seek;
use std::io::Write;

use anyhow::ensure;
use itertools::Itertools;
use msi::Package;
use tracing::debug;
use tracing::trace;

use crate::tables::dao::IsDao;

pub(crate) trait MsiBuilderTable: Default {
    type TableValue: IsDao;
    // Handled by boilerplate macro defined below
    fn items(&self) -> &Vec<Self::TableValue>;
    fn items_mut(&mut self) -> &mut Vec<Self::TableValue>;

    /// Utilized when creating the MSI using the `msi` crate.
    fn name(&self) -> &'static str;
    fn columns(&self) -> Vec<msi::Column>;

    fn add(&mut self, dao: Self::TableValue) -> anyhow::Result<()> {
        ensure!(!self.contains(&dao), "TEMPERROR");
        self.items_mut().push(dao);
        Ok(())
    }

    fn add_all(&mut self, daos: Vec<Self::TableValue>) -> anyhow::Result<()> {
        daos.into_iter()
            .map(|dao| self.add(dao))
            .collect::<anyhow::Result<Vec<()>>>()?;
        Ok(())
    }

    fn is_empty(&self) -> bool {
        self.items().is_empty()
    }

    fn len(&self) -> usize {
        self.items().len()
    }

    fn rows(&self) -> Vec<Vec<msi::Value>> {
        self.items().iter().map(IsDao::to_row).collect_vec()
    }

    fn contains(&self, other: &Self::TableValue) -> bool {
        self.items().iter().find(|entry| entry.conflicts(other)).is_some()
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
        fn items(&self) -> &Vec<Self::TableValue> {
            &self.0
        }

        fn items_mut(&mut self) -> &mut Vec<Self::TableValue> {
            &mut self.0
        }
    };
}
