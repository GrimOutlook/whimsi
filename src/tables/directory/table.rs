use anyhow::bail;
use anyhow::ensure;
use itertools::Itertools;
use thiserror::Error;

use super::dao::DirectoryDao;
use crate::constants::*;
use crate::msitable_boilerplate;
use crate::tables::builder_table::MsiBuilderTable;
use crate::types::column::default_dir::DefaultDir;
use crate::types::column::identifier::Identifier;
use crate::types::column::identifier::ToIdentifier;
use crate::types::properties::system_folder::SystemFolder;

#[derive(Clone, Debug, PartialEq)]
pub struct DirectoryTable(Vec<DirectoryDao>);
impl MsiBuilderTable for DirectoryTable {
    type TableValue = DirectoryDao;

    // Boilderplate needed to access information on the inner object
    msitable_boilerplate!();

    fn name(&self) -> &'static str {
        "Directory"
    }

    fn columns(&self) -> Vec<msi::Column> {
        vec![
            msi::Column::build("Directory")
                .primary_key()
                .id_string(IDENTIFIER_MAX_LEN),
            msi::Column::build("Directory_Parent")
                .nullable()
                .id_string(IDENTIFIER_MAX_LEN),
            msi::Column::build("DefaultDir")
                .category(msi::Category::DefaultDir)
                .string(DEFAULT_DIR_MAX_LEN),
        ]
    }

    fn rows(&self) -> Vec<Vec<msi::Value>> {
        self.values().iter().map(DirectoryDao::to_row).collect_vec()
    }

    fn contains(&self, dao: &DirectoryDao) -> bool {
        // NOTE: We purposefully allow entries that have the same DefaultDir and
        // are contained by the same parent because you can assign
        // different components to these entries if you want
        // both components to install to the same location but based on separate
        // criteria.
        self.0
            .iter()
            .find(|entry| entry.directory() == dao.directory())
            .is_some()
    }

    fn add(&mut self, dao: Self::TableValue) -> anyhow::Result<()> {
        // TODO: Create actual error for directory ID collision.
        ensure!(!self.contains(&dao), "TEMPERROR");
        self.0.push(dao);
        Ok(())
    }
}

impl Default for DirectoryTable {
    fn default() -> Self {
        let v = vec![SystemFolder::TARGETDIR.into()];
        Self(v)
    }
}

impl DirectoryTable {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn entry_with_id(
        &self,
        identifier: &Identifier,
    ) -> Option<&DirectoryDao> {
        self.0.iter().find(|d| d.directory().to_identifier() == *identifier)
    }

    pub fn entries_with_parent(
        &self,
        parent_id: &Identifier,
    ) -> Vec<&DirectoryDao> {
        self.0
            .iter()
            .filter(|d| d.parent().to_identifier() == *parent_id)
            .collect_vec()
    }

    pub fn has_directory_id(&self, identifier: &Identifier) -> bool {
        self.entry_with_id(identifier).is_some()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Debug, Error)]
pub enum DirectoryTableError {
    #[error("Parent ID {parent_id} is not in DirectoryTable")]
    ParentDirectoryNotPresent { parent_id: Identifier },
    #[error(
        "Directory with ID {parent_id} already contains subdirectory with name {name:?}"
    )]
    DirectoryNameCollision { parent_id: Identifier, name: DefaultDir },
}

#[cfg(test)]
mod test {
    use std::io::Cursor;
    use std::str::FromStr;

    use msi::PackageType;
    use msi::Select;

    use super::DirectoryTable;
    use crate::tables::builder_table::MsiBuilderTable;
    use crate::tables::directory::dao::DirectoryDao;
    use crate::types::column::default_dir::DefaultDir;
    use crate::types::column::identifier::Identifier;
    use crate::types::helpers::filename::Filename;
    use crate::types::properties::system_folder::SystemFolder;

    #[test]
    fn write_to_package() {
        let mut package = msi::Package::create(
            PackageType::Installer,
            Cursor::new(Vec::new()),
        )
        .unwrap();
        let mut table = DirectoryTable::default();
        let parent = SystemFolder::ProgramFilesFolder;
        table.add(parent.into());
        table.add(DirectoryDao::new(
            DefaultDir::Filename(Filename::from_str("test").unwrap()),
            Identifier::from_str("test_id").unwrap(),
            parent,
        ));
        table.write_to_package(&mut package).unwrap();

        let directory_table = package.get_table("Directory").unwrap();
        assert!(
            directory_table.has_column("Directory"),
            "MSI Directory Table doesn't have `Directory` column"
        );
        assert!(
            directory_table.has_column("Directory_Parent"),
            "MSI Directory Table doesn't have `Directory_Parent` column"
        );
        assert!(
            directory_table.has_column("DefaultDir"),
            "MSI Directory Table doesn't have `DefaultDir` column"
        );
        let rows = package.select_rows(Select::table("Directory")).unwrap();
        // TARGETDIR is always in the table by default so add 1 to the total we
        // expect
        assert_eq!(rows.len(), 2 + 1, "Directory table row count mismatch");
    }
}
