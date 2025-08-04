// Populates the `Directory` table

use std::{cell::RefCell, io::Cursor, rc::Rc, sync::Arc};

use anyhow::{Context, Result};
use camino::{Utf8DirEntry, Utf8PathBuf};
use msi::{Category, Column, Insert, Package, Value};

use crate::{
    builder::{
        tables::{DEFAULT_DIR_LEN, ID_STRING_LEN},
        MsiPackage,
    },
    models::directory::MsiDirectory,
};

use super::Tables;

pub(crate) struct DirectoryTable {
    pub(crate) package: Rc<RefCell<MsiPackage>>,
    directories: Vec<MsiDirectory>,
}

impl DirectoryTable {
    pub fn add(&mut self, directories: &[MsiDirectory]) -> Result<()> {
        let rows = directories
            .iter()
            .map(|dir| {
                vec![
                    Value::from(dir.id().to_string()),
                    match &dir.parent_id() {
                        Some(p) => Value::from(p.to_string()),
                        None => Value::Null,
                    },
                    Value::from(dir.name().to_string()),
                ]
            })
            .collect();

        self.insert(rows)?;

        Ok(())
    }
}

impl Tables for DirectoryTable {
    /// Column information can be found here:
    /// https://learn.microsoft.com/en-us/windows/win32/msi/directory-table#columns
    fn columns(&self) -> Vec<Column> {
        vec![
            Column::build("Directory")
                .primary_key()
                .id_string(ID_STRING_LEN),
            Column::build("Directory_Parent")
                .nullable()
                .id_string(ID_STRING_LEN),
            Column::build("DefaultDir")
                .category(Category::DefaultDir)
                .string(DEFAULT_DIR_LEN),
        ]
    }

    fn name(&self) -> &'static str {
        "Directory"
    }

    fn package(&self) -> Rc<RefCell<MsiPackage>> {
        self.package.clone()
    }

    fn default_data(&self) -> Option<Vec<Vec<Value>>> {
        todo!()
    }
}
