// Populates the `File` table

use anyhow::{Context, Result};
use msi::{Category, Column, Insert, Value};
use uuid::Uuid;

use crate::{builder::MsiPackage, models::file::MsiFile};

const TABLE_NAME: &str = "Component";

pub fn populate_component_table(package: &mut MsiPackage, files: &[MsiFile]) -> Result<()> {
    create_component_table(package)?;

    let query = Insert::into(TABLE_NAME).rows(
        files
            .iter()
            .map(|file| {
                vec![
                    Value::from(file.component_id().to_string()),
                    Value::from(Uuid::new_v4().to_string()),
                    Value::from(file.name().to_string()),
                    Value::from(if *file.vital() { 16 } else { 0 }),
                ]
            })
            .collect(),
    );

    // NOTE: This needs to come before calling insert_rows since it takes ownership of query.
    let query_str = query.to_string();
    package.insert_rows(query).context(format!(
        "Inserting row into component table using query [{query_str}]"
    ))?;

    Ok(())
}

fn create_component_table(package: &mut MsiPackage) -> Result<()> {
    package
        .create_table(
            TABLE_NAME,
            vec![
                Column::build("Component").primary_key().id_string(72),
                Column::build("ComponentId")
                    .category(Category::Guid)
                    .nullable()
                    .string(38),
                Column::build("Directory_").id_string(72),
                Column::build("Attributes").int16(),
                Column::build("Condition")
                    .nullable()
                    .category(Category::Condition)
                    .string(255),
                Column::build("KeyPath").nullable().id_string(72),
            ],
        )
        .context(format!("Creating {TABLE_NAME} table"))?;

    Ok(())
}
