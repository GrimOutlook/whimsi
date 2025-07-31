// Populates the `Directory` table

use anyhow::{Context, Result};
use msi::{Category, Column, Insert, Value};

use crate::{builder::MsiCursor, models::directory::Directory};

pub fn populate_directory_table(package: &mut MsiCursor, directories: &[Directory]) -> Result<()> {
    create_directory_table(package)?;

    let query = Insert::into("Directory").rows(
        directories
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
            .collect(),
    );

    // NOTE: This needs to come before calling insert_rows since it takes ownership of query.
    let query_str = query.to_string();
    package.insert_rows(query).context(format!(
        "Inserting row into directory table using query [{query_str}]"
    ))?;

    Ok(())
}

fn create_directory_table(package: &mut MsiCursor) -> Result<()> {
    package
        .create_table(
            "Directory",
            vec![
                Column::build("Directory").primary_key().id_string(72),
                Column::build("Directory_Parent").nullable().id_string(72),
                Column::build("DefaultDir")
                    .category(Category::DefaultDir)
                    .string(255),
            ],
        )
        .context("Creating Directory table")?;

    Ok(())
}
