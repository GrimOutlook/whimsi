use std::{
    fs::{read_to_string, File},
    io::{Cursor, Write},
    rc::Rc,
};

use anyhow::{bail, ensure};
use anyhow::{Context, Result};
use camino::Utf8PathBuf;
use msi::{Package, PackageType};
use roxygen::roxygen;
use tracing::info;

use crate::{
    enums::system_folder::SystemFolder,
    models::sequencer::Sequencer,
    scan,
    tables::{self, directory::DirectoryTable},
};

// Make a shorthand way to refer to the package cursor for brevity.
pub(crate) type MsiPackage = Package<Cursor<Vec<u8>>>;

pub struct MsiBuilder<'a> {
    package: MsiPackage,
    directory_table: DirectoryTable<'a>,
}

impl<'a> MsiBuilder<'a> {
    /// Create a new, emoty, MSI database to manipulate.
    pub fn new() -> Result<Self> {
        // Create an empty MSI that we can populate.
        let cursor = Cursor::new(Vec::new());
        let package =
            Package::create(PackageType::Installer, cursor).context("Creating MSI installer")?;

        Ok(Self {
            package,
            component_sequencer: Sequencer::new("component"),
            file_sequencer: Sequencer::new("file"),
            directory_sequencer: Sequencer::new("directory"),
        })
    }

    fn set_author(&mut self, author: String) {
        self.package.summary_info_mut().set_author(author);
    }

    #[roxygen]
    fn add_path(
        &mut self,
        /// The path you want to add to the MSI. Can be to a file or a directory.
        path: &Utf8PathBuf,
        /// The base path/location of where the files and directories from the given path are to be
        /// moved. This must be a Microsoft defined property so arbitrary paths are not allowed.
        destination_base: SystemFolder,
        /// Path to append to the base path in order to get the full path. This is the full path
        /// where the files are to be placed upon install.
        destination_suffix: Option<String>,
    ) -> Result<()> {
        let (directories, files) = scan::scan_path(path)?;

        self.directory_table.add_directories(&directories)?;
        todo!();
        // tables::component::populate_component_table(&mut package, &files)?;
        // tables::file::populate_file_table(&mut package, &files)?;
    }

    /// Returns the in-memory database that was created by the commands passed to the builder.
    pub fn finish(self) -> Cursor<Vec<u8>> {
        self.package.into_inner().unwrap()
    }

    /// Write the in-memory MSI data to the output location
    pub fn write(self, output_path: &Utf8PathBuf) -> Result<()> {
        let cursor = self.finish();
        let mut file = File::create(output_path).context(format!(
            "Failed to open output path {output_path} for writing"
        ))?;

        file.write_all(cursor.get_ref()).context(format!(
            "Failed to write MSI data to location {output_path}"
        ))?;
        info!("Wrote MSI to {}", output_path);
        Ok(())
    }
}

// impl TryFrom<MsiConfig> for MsiBuilder {
// type Error = anyhow::Error;
// fn try_from(config: MsiConfig) -> std::result::Result<Self, Self::Error> {
//         todo!();
//         info!("Building MSI at output path {}", config.output_path);
//         // Validate paths before continuing
//         validate_paths(config_path, output_path)?;
//         // Read the config from the passed in path
//         let raw_config =
//             read_to_string(config_path).context(format!("Failed to open config {config_path}"))?;
//         let config: Rc<MsiConfig> = toml::from_str::<MsiConfig>(&raw_config)
//             .context(format!("Failed to parse config toml {config_path}"))?
//             .into();
//
//         // Check the config for common errors
//         check_config(&config)?;
//
//         // Create an empty MSI that we can populate.
//         let cursor = Cursor::new(Vec::new());
//         let mut package =
//             Package::create(PackageType::Installer, cursor).context("Creating MSI installer")?;
//
//         // Set the author
//         set_author(&mut package, config.clone());
//
//         // Add the files from the input directory
//         let (directories, files) = scan::scan_paths(config.clone(), input_directory)?;
//
//         tables::directory::populate_directory_table(&mut package, &directories)?;
//         tables::component::populate_component_table(&mut package, &files)?;
//         tables::file::populate_file_table(&mut package, &files)?;
//
//         Ok(package)
//     }
// }
