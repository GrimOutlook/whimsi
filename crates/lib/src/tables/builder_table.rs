use std::io::Read;
use std::io::Seek;
use std::io::Write;

use anyhow::anyhow;
use anyhow::ensure;
use itertools::Itertools;
use msi::Package;
use tracing::debug;
use tracing::trace;

use crate::tables::dao::MsiDao;
use crate::types::helpers::primary_identifier::PrimaryIdentifier;

/// Structs that implement this trait are able to write to a table in an
/// msi::Package.
pub trait PackageTable: DaoList {
    fn name(&self) -> &'static str;
    fn columns(&self) -> Vec<msi::Column>;
    fn primary_key_indices(&self) -> Vec<usize>;

    fn rows(&self) -> Vec<Vec<msi::Value>> {
        self.entries().into_iter().map(MsiDao::to_row).collect_vec()
    }

    /// Write the columns contained in the table to the package.
    fn write_to_package<F: std::io::Read + std::io::Write + std::io::Seek>(
        &self,
        package: &mut msi::Package<F>,
    ) -> anyhow::Result<()> {
        debug!("Writing {}Table to package", self.name());
        let columns = self.columns();

        if !package.has_table(self.name()) {
            package.create_table(self.name(), columns)?;
        }

        let rows = self.rows();
        if rows.is_empty() {
            return Ok(());
        }

        trace!("Inserting rows into {}Table:", self.name());
        rows.clone()
            .iter()
            .enumerate()
            .for_each(|(index, r)| trace!("{index}: {r:?}"));
        let query = msi::Insert::into(self.name()).rows(rows);
        package.insert_rows(query)?;
        Ok(package.flush()?)
    }
}

pub trait DaoList {
    type Dao: MsiDao + PrimaryIdentifier + PartialEq;

    fn entries(&self) -> &Vec<Self::Dao>;
    fn entries_mut(&mut self) -> &mut Vec<Self::Dao>;

    fn is_empty(&self) -> bool {
        self.entries().is_empty()
    }

    fn len(&self) -> usize {
        self.entries().len()
    }

    fn contains(&self, other: &Self::Dao) -> bool {
        self.entries().iter().any(|entry| entry.conflicts_with(other))
    }

    /// Add the given `Dao` to the list.
    ///
    /// # Returns
    /// - `Ok(())`: If the entry does not share the same primary key(s) as an
    /// item already in the list.
    /// - `Err(E)`: If the entry shares the same primary key(s) as an item
    /// already in the list
    fn add(&mut self, entry: Self::Dao) -> anyhow::Result<()> {
        ensure!(
            !self.contains(&entry),
            "Input conflicts with value already present."
        );
        self.entries_mut().push(entry);
        Ok(())
    }

    fn add_all(&mut self, entries: Vec<Self::Dao>) -> anyhow::Result<()> {
        entries
            .into_iter()
            .map(|entry| self.add(entry))
            .collect::<anyhow::Result<Vec<()>>>()?;
        Ok(())
    }
}
