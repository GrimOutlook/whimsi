use std::{
    fs::{File, read_to_string},
    io::{Cursor, Write},
    rc::Rc,
};

use anyhow::{Context, Result};
use anyhow::{bail, ensure};
use camino::Utf8PathBuf;
use msi::{Package, PackageType};
use tracing::info;

use crate::{config::MsiConfig, scan, tables};

// Make a shorthand way to refer to the package cursor for brevity.
pub(crate) type Msi = Package<Cursor<Vec<u8>>>;

pub(crate) fn build(
    config_path: &Utf8PathBuf,
    input_directory: &Utf8PathBuf,
    output_path: &Utf8PathBuf,
) -> anyhow::Result<()> {
    info!("Building MSI at output path {}", output_path);
    // Validate paths before continuing
    validate_paths(config_path, input_directory, output_path)?;
    // Read the config from the passed in path
    let raw_config = read_to_string(config_path)
        .context(format!("Failed to open config {config_path}"))?;
    let config: Rc<MsiConfig> = toml::from_str::<MsiConfig>(&raw_config)
        .context(format!("Failed to parse config toml {config_path}"))?
        .into();

    // Check the config for common errors
    check_config(&config)?;

    // Create an empty MSI that we can populate.
    let cursor = Cursor::new(Vec::new());
    let mut package = Package::create(PackageType::Installer, cursor)
        .context("Creating MSI installer")?;

    // Set the author
    set_author(&mut package, config.clone());

    // Add the files from the input directory
    let (directories, files) =
        scan::scan_paths(config.clone(), input_directory)?;

    tables::directory::populate_directory_table(&mut package, &directories)?;
    tables::component::populate_component_table(&mut package, &files)?;
    tables::file::populate_file_table(&mut package, &files)?;

    write_msi(package, output_path)
}

fn set_author(package: &mut Msi, config: Rc<MsiConfig>) {
    package
        .summary_info_mut()
        .set_author(config.summary_info.author.clone().unwrap_or_default());
}

fn write_msi(package: Msi, output_path: &Utf8PathBuf) -> Result<()> {
    let cursor = package.into_inner().unwrap();
    let mut file = File::create(output_path).context(format!(
        "Failed to open output path {output_path} for writing"
    ))?;

    file.write_all(cursor.get_ref())
        .context(format!("Failed to write MSI to location {output_path}"))?;
    info!("Wrote MSI to {}", output_path);
    Ok(())
}

fn check_config(config: &MsiConfig) -> Result<()> {
    if config.default_files.is_none() && config.explicit_files.is_none() {
        bail!(
        "No files specified for MSI.
        Files should be specified under `[default_files]` and `[explicit_files]` sections.
        To disable this error use the `--no-files` flag."
        );
    }

    // TODO: Do I really need this check?
    // if let Some(default_files) = &config.default_files {
    //     if default_files.program_files.is_none() && default_files.program_files_32.is_none() {
    //         let msg = error!("No program files found in `[default_files]` section.");
    //         error!(
    //             "`program_files` or `program_files_32` must be present if `[default_files]` section is used."
    //         );
    //         return Err(MsiError::short(msg));
    //     }
    // }

    Ok(())
}

pub(crate) fn validate_paths(
    config_path: &Utf8PathBuf,
    input_directory: &Utf8PathBuf,
    output_path: &Utf8PathBuf,
) -> Result<()> {
    // Convert the string (representing the path to scan) into an absolute path.
    let full_path = camino::absolute_utf8(Utf8PathBuf::from(&output_path))
        .context(format!(
            "Get full path for the passed in output path [{output_path}]"
        ))?;

    // Since parent returns None when you are at the root folder, it's fine to
    // just use the full path if we hit None as this should just end up being
    // `/` or `C:\` which is valid.
    let output_parent_dir = full_path.as_path().parent().unwrap_or(&full_path);

    ensure!(config_path.exists(), "Config path {config_path} does not exist");
    ensure!(config_path.is_file(), "Config path {config_path} is not a file");
    ensure!(
        input_directory.exists(),
        "Input directory {input_directory} does not exist"
    );
    ensure!(
        input_directory.is_dir(),
        "Input directory {input_directory} is not a directory"
    );
    ensure!(
        output_path.parent().is_some(),
        "Output path {output_path} is not valid a valid filepath."
    );
    ensure!(
        output_parent_dir.is_dir(),
        "Output parent directory {output_parent_dir} is not a valid directory."
    );

    Ok(())
}
