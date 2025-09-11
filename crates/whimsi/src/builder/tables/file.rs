// Populates the `File` table

use anyhow::{Context, Result};
use whimsi_msi::{Category, Column};

use crate::{builder::MsiPackage, models::file::MsiFile};

pub fn populate_file_table(package: &mut MsiPackage, files: &[MsiFile]) -> Result<()> {
    create_file_table(package)?;
    todo!();

    // let query = Insert::into("File").rows(
    //     files
    //         .iter()
    //         .map(|file| {
    //             vec![
    //                 Value::from(file.id().to_string()),
    //                 match &dir.parent_id() {
    //                     Some(p) => Value::from(p.to_string()),
    //                     None => Value::Null,
    //                 },
    //                 Value::from(dir.name().to_string()),
    //             ]
    //         })
    //         .collect(),
    // );

    // if let Err(err) = package.insert_rows(query) {
    //     return Err(MsiError::nested("Failed to insert row into table", err));
    // };

    Ok(())
}

fn create_file_table(package: &mut MsiPackage) -> Result<()> {
    package
        .create_table(
            "File",
            vec![
                Column::build("File").primary_key().id_string(72),
                Column::build("Component_").id_string(72),
                Column::build("FileName")
                    .category(Category::Filename)
                    .string(255),
                Column::build("FileSize")
                    .category(Category::DoubleInteger)
                    .int16(),
                Column::build("Version")
                    .nullable()
                    .category(Category::Version)
                    .string(72),
                Column::build("Language")
                    .nullable()
                    .category(Category::Language)
                    .string(20),
                Column::build("Attributes").nullable().int16(),
                Column::build("Sequence").int16(),
            ],
        )
        .context("Creating File table")?;

    Ok(())
}
