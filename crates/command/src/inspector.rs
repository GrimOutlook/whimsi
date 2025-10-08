use anyhow::{bail, Context, Result};
use camino::Utf8PathBuf;
use cli_table::{Cell, CellStruct, Style, Table};
use flexstr::{LocalStr, SharedStr};
use itertools::Itertools;
use msi::{Package, Select};
use std::fs::File;
use tracing::{debug, info};

use crate::Listable;

pub(crate) fn inspect(input_file: &Utf8PathBuf, list_item: Listable) -> Result<String> {
    info!("Reading MSI {}", input_file);
    validate_paths(input_file)?;

    let mut msi = msi::open_rw(input_file).context(format!("Failed to open MSI {input_file}"))?;

    match list_item {
        Listable::Author => list_author(msi),
        Listable::Tables => list_tables(msi),
        Listable::NonEmptyTables => list_non_empty_tables(&mut msi),
        Listable::TableColumns { table } => list_table_columns(msi, table),
        Listable::TableContents { table } => list_table_contents(&mut msi, table),
    }
}

pub(crate) fn validate_paths(input_file: &Utf8PathBuf) -> Result<()> {
    if !input_file.exists() {
        bail!("Input file {} does not exist", input_file)
    }
    if !input_file.is_file() {
        bail!("Input file {} is not a file", input_file)
    }
    Ok(())
}

fn list_author(msi: Package<File>) -> Result<String> {
    debug!("Listing author of MSI");
    let author = msi.summary_info().author().unwrap_or_default();
    Ok(author.to_owned())
}

fn list_tables(msi: Package<File>) -> Result<String> {
    debug!("Listing tables in MSI");
    let tables = msi.tables().map(|t| t.name()).collect::<Vec<&str>>();
    Ok(tables.join("\n"))
}

fn list_non_empty_tables(msi: &mut Package<File>) -> Result<String> {
    debug!("Listing non-empty tables in MSI");
    let all_tables = msi.tables().cloned().collect_vec();
    let non_empty_tables = all_tables
        .iter()
        .filter(|table| {
            if let Ok(rows) = msi.select_rows(Select::table(table.name().to_string())) {
                return !rows.collect_vec().is_empty();
            }
            false
        })
        .map(|table| table.name())
        .collect_vec();
    Ok(non_empty_tables.join("\n"))
}

/// List the columns present in the given table
fn list_table_columns(msi: Package<File>, table: SharedStr) -> Result<String> {
    debug!("Listing the columns of table {} in MSI", table);
    let table = msi
        .get_table(&table)
        .context(format!("Table {table} could not be found in MSI"))?;

    let columns = table.columns();

    let contents: Vec<Vec<CellStruct>> = columns
        .iter()
        .map(|c| {
            vec![
                c.name().cell(),
                c.coltype().to_string().cell(),
                format!("{:?}", c.category()).cell(),
                c.is_nullable().cell(),
            ]
        })
        .collect();

    let table_columns = ["Column", "Type", "Category", "Nullable"];

    let print_table = contents
        .table()
        .title(table_columns.iter().map(|c| c.cell().bold(true)))
        .bold(true);

    Ok(print_table
        .display()
        .context("Displaying table columns")?
        .to_string())
}

/// List the contents of the given table
fn list_table_contents(msi: &mut Package<File>, table_name: SharedStr) -> Result<String> {
    debug!("Listing the contents of table {} in MSI", table_name);

    let rows = msi
        .select_rows(Select::table(table_name.to_string()))
        .context("Getting table rows")?;

    let columns = rows
        .columns()
        .iter()
        .map(|c| c.name().to_string())
        .collect::<Vec<String>>();

    let contents: Vec<Vec<CellStruct>> = rows
        .map(|r| {
            columns
                .iter()
                .map(|c| r[c.as_str()].to_string().cell())
                .collect()
        })
        .collect();

    let table = contents
        .table()
        .title(columns.iter().map(|c| c.cell().bold(true)))
        .bold(true);

    Ok(table
        .display()
        .context("Failed to display table")?
        .to_string())
}
