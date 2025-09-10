use anyhow::bail;
use anyhow::ensure;
use itertools::Itertools;
use thiserror::Error;

use super::dao::DirectoryDao;
use crate::constants::*;
use crate::define_generator_table;
use crate::define_identifier_generator;
use crate::define_specific_identifier;
use crate::define_specific_identifier_parsing;
use crate::implement_id_generator_for_table;
use crate::msi_list_boilerplate;
use crate::msi_table_boilerplate;
use crate::tables::builder_list::MsiBuilderList;
use crate::tables::builder_table::MsiBuilderTable;
use crate::tables::directory::directory_identifier::DirectoryIdentifier;
use crate::tables::id_generator_builder_list::IdGeneratorBuilderList;
use crate::types::column::default_dir::DefaultDir;
use crate::types::column::identifier::Identifier;
use crate::types::column::identifier::ToIdentifier;
use crate::types::properties::system_folder::SystemFolder;

define_identifier_generator!(Directory);
define_generator_table!(
    Directory,
    vec![
        msi::Column::build("Directory")
            .primary_key()
            .id_string(DEFAULT_IDENTIFIER_MAX_LEN),
        msi::Column::build("Directory_Parent")
            .nullable()
            .id_string(DEFAULT_IDENTIFIER_MAX_LEN),
        msi::Column::build("DefaultDir")
            .category(msi::Category::DefaultDir)
            .string(DEFAULT_DIR_MAX_LEN),
    ]
);

impl DirectoryTable {
    pub fn new(
        identifiers: std::rc::Rc<std::cell::RefCell<Vec<Identifier>>>,
    ) -> Self {
        let entries = Vec::new();
        let generator = DirectoryIdGenerator::from(identifiers);
        let mut table = Self { entries, generator };
        IdGeneratorBuilderList::add(
            &mut table,
            DirectoryDao::from(SystemFolder::TARGETDIR),
        )
        .expect("Failed to add default directtories to Directory table.");
        table
    }

    pub fn entry_with_id(
        &self,
        identifier: &Identifier,
    ) -> Option<&DirectoryDao> {
        self.entries
            .iter()
            .find(|d| d.directory().to_identifier() == *identifier)
    }

    pub fn entries_with_parent(
        &self,
        parent_id: &Identifier,
    ) -> Vec<&DirectoryDao> {
        self.entries
            .iter()
            .filter(|d| d.parent().to_identifier() == *parent_id)
            .collect_vec()
    }

    pub fn has_directory_id(&self, identifier: &Identifier) -> bool {
        self.entry_with_id(identifier).is_some()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }
}

msi_list_boilerplate!(DirectoryTable, DirectoryDao);
implement_id_generator_for_table!(DirectoryTable, DirectoryIdGenerator);

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
    use std::cell::RefCell;
    use std::io::Cursor;
    use std::rc::Rc;
    use std::str::FromStr;

    use msi::PackageType;
    use msi::Select;

    use super::DirectoryTable;
    use crate::tables::builder_list::MsiBuilderList;
    use crate::tables::builder_table::MsiBuilderTable;
    use crate::tables::directory::dao::DirectoryDao;
    use crate::types::column::default_dir::DefaultDir;
    use crate::types::column::filename::Filename;
    use crate::types::column::identifier::Identifier;
    use crate::types::properties::system_folder::SystemFolder;

    #[test]
    fn write_to_package() {
        let mut package = msi::Package::create(
            PackageType::Installer,
            Cursor::new(Vec::new()),
        )
        .unwrap();
        let mut table = DirectoryTable::new(Rc::new(RefCell::new(Vec::new())));
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
