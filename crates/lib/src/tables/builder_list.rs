use anyhow::ensure;

use crate::{
    tables::dao::MsiDao,
    types::helpers::to_unique_msi_identifier::PrimaryIdentifier,
};

pub(crate) trait MsiBuilderList {
    type ListValue: MsiDao + PrimaryIdentifier;

    // Handled by boilerplate macro defined below
    fn entries(&self) -> &Vec<Self::ListValue>;
    fn entries_mut(&mut self) -> &mut Vec<Self::ListValue>;

    fn add(&mut self, entry: Self::ListValue) -> anyhow::Result<()> {
        ensure!(
            !self.contains(&entry),
            "Input conflicts with value already present."
        );
        self.entries_mut().push(entry);
        Ok(())
    }

    fn add_all(&mut self, entries: Vec<Self::ListValue>) -> anyhow::Result<()> {
        entries
            .into_iter()
            .map(|entry| self.add(entry))
            .collect::<anyhow::Result<Vec<()>>>()?;
        Ok(())
    }

    fn is_empty(&self) -> bool {
        self.entries().is_empty()
    }

    fn len(&self) -> usize {
        self.entries().len()
    }

    fn contains(&self, other: &Self::ListValue) -> bool {
        self.entries().iter().find(|entry| entry.conflicts(other)).is_some()
    }
}
