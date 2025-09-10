use std::io::Read;
use std::io::Seek;
use std::io::Write;

use anyhow::ensure;
use itertools::Itertools;
use msi::Package;
use tracing::debug;
use tracing::trace;

use crate::tables::builder_list::MsiBuilderList;
use crate::tables::builder_list_entry::MsiBuilderListEntry;
use crate::tables::dao::IsDao;
use crate::types::helpers::to_unique_msi_identifier::ToUniqueMsiIdentifier;

pub(crate) trait MsiBuilderTable: MsiBuilderList {
    type TableValue: IsDao + ToUniqueMsiIdentifier + MsiBuilderListEntry;

    /// Utilized when creating the MSI using the `msi` crate.
    fn name(&self) -> &'static str;
    fn columns(&self) -> Vec<msi::Column>;
    fn entries(&self) -> &Vec<Self::TableValue>;
    fn entries_mut(&mut self) -> &mut Vec<Self::TableValue>;

    fn rows(&self) -> Vec<Vec<msi::Value>> {
        MsiBuilderTable::entries(self).iter().map(IsDao::to_row).collect_vec()
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
macro_rules! msi_table_boilerplate {
    () => {
        fn entries(&self) -> &Vec<Self::TableValue> {
            &self.entries
        }

        fn entries_mut(&mut self) -> &mut Vec<Self::TableValue> {
            &mut self.entries
        }
    };
}
