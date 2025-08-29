use anyhow::{bail, ensure};
use itertools::Itertools;
use thiserror::Error;

use crate::msitable_boilerplate;
use crate::tables::builder_table::MsiBuilderTable;
use crate::types::column::default_dir::DefaultDir;
use crate::types::column::identifier::Identifier;
use crate::types::properties::system_folder::SystemFolder;

use super::dao::DirectoryDao;

#[derive(Clone, Debug, PartialEq)]
pub struct DirectoryTable(Vec<DirectoryDao>);
impl MsiBuilderTable for DirectoryTable {
    type TableValue = DirectoryDao;

    // Boilderplate needed to access information on the inner object
    msitable_boilerplate!();

    fn name() -> &'static str {
        "Directory"
    }

    fn default_values() -> Vec<Self::TableValue> {
        todo!()
    }

    fn add(&mut self, dao: Self::TableValue) -> anyhow::Result<()> {
        let parent_id = dao.parent();
        // Verify that the parent directory is already in the directories table.
        // If the parent ID is associated with a SystemFolder, make sure that system folder is in
        // the table.
        if self.entry_with_id(parent_id).is_none() {
            if let Some(sys_folder) = parent_id.as_system_folder() {
                self.add(DirectoryDao::from(sys_folder))?;
            } else {
                bail!(DirectoryTableError::ParentDirectoryNotPresent {
                    parent_id: parent_id.clone()
                })
            }
        }

        // Check that the new item isn't already in the parent directory. Can only check
        // against the DAO names, as the identifiers are able to be randomly generated.
        ensure!(
            !self
                .entries_with_parent(parent_id)
                .iter()
                .any(|d| d.default_dir() == dao.default_dir()),
            DirectoryTableError::DirectoryNameCollision {
                parent_id: parent_id.clone(),
                name: dao.default_dir().clone()
            }
        );

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
    pub fn entry_with_id(&self, identifier: &Identifier) -> Option<&DirectoryDao> {
        self.0.iter().find(|d| d.directory() == identifier)
    }

    pub fn entries_with_parent(&self, parent_id: &Identifier) -> Vec<&DirectoryDao> {
        self.0
            .iter()
            .filter(|d| d.parent() == parent_id)
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
    #[error("Directory with ID {parent_id} already contains subdirectory with name {name:?}")]
    DirectoryNameCollision {
        parent_id: Identifier,
        name: DefaultDir,
    },
}
