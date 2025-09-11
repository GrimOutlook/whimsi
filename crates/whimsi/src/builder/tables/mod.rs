use std::{cell::RefCell, default, io::Cursor, rc::Rc};

use ambassador::{delegatable_trait, Delegate};
use anyhow::{Context, Result};
use directory::DirectoryTable;
use whimsi_msi::{Column, Insert, Package, Rows, Value};
use strum::EnumDiscriminants;

use super::MsiPackage;

pub(crate) mod component;
pub(crate) mod directory;
pub(crate) mod file;

/// NOTE: Because Microsoft's documentation is subpar, these values are derived by me running
/// `inspect table_columns [TABLE]` and seeing the length allocated in the database of the
/// [Microsoft Teams
/// MSI](https://learn.microsoft.com/en-us/microsoftteams/msi-deployment)
/// that I figured would have the correct values since Microsoft packaged it themselves. I cannot
/// find values for these anywhere in the MS documentation but if anyone finds where they are
/// actually specified I would be quite pleased. I simply refuse to believe that they left the
/// implementation of the length of the data in columns up to implementation.
const ID_STRING_LEN: usize = 72;
const DEFAULT_DIR_LEN: usize = 255;

pub trait TableTrait {
    fn name() -> &'static str;
    fn columns() -> Vec<Column>;
    fn default_data() -> Option<Vec<Vec<Value>>>;

    fn insert(package: &mut MsiPackage, rows: Vec<Vec<Value>>) -> Result<()> {
        let query = Insert::into(Self::name()).rows(rows);

        // NOTE: This needs to come before calling insert_rows since it takes ownership of query.
        let query_str = query.to_string();
        package.insert_rows(query).context(format!(
            "Inserting row into directory table using query [{query_str}]"
        ))
    }
    fn init_table(package: &mut MsiPackage) -> Result<()> {
        package
            .create_table(Self::name(), Self::columns())
            .context("Creating Directory table")?;

        // Initialize the table with the values that will always exist within.
        if let Some(default_data) = Self::default_data() {
            Self::insert(package, default_data)?;
        }

        Ok(())
    }
}
