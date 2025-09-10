mod tables;

use std::{
    cell::RefCell,
    fs::{read_to_string, File},
    io::{Cursor, Write},
    rc::Rc,
    sync::Arc,
};

use anyhow::{bail, ensure};
use anyhow::{Context, Result};
use camino::Utf8PathBuf;
use msi::{Package, PackageType};
use roxygen::roxygen;
use tables::directory::DirectoryTable;
use tracing::info;

use crate::{
    enums::system_folder::SystemFolder,
    models::{
        directory::{self, MsiDirectory},
        sequencer::Sequencer,
    },
};

// Make a shorthand way to refer to the package cursor for brevity.
pub(crate) type MsiPackage = Package<Cursor<Vec<u8>>>;

pub struct MsiBuilder {
    package: Rc<RefCell<MsiPackage>>,
}

impl MsiBuilder {
    /// Create a new, emoty, MSI database to manipulate.
    pub fn new() -> Result<Self> {
        // Create an empty MSI that we can populate.
        let cursor = Cursor::new(Vec::new());
        let package = Rc::new(RefCell::new(
            Package::create(PackageType::Installer, cursor).context("Creating MSI installer")?,
        ));

        Ok(Self {
            package: package.clone(),
        })
    }

    pub fn set_author(&mut self, author: String) {
        self.package
            .borrow_mut()
            .summary_info_mut()
            .set_author(author);
    }

    /// WARN: Currently only paths from `SystemFolder`s are supported. In the future I would like to
    /// support adding directories to non-systemfolder ID'd directories if they are already present
    /// in the database but this is not implemented yet.
    /// WARN: Only recursive path adding is currently supported.
    /// TODO: Implement adding directories/files to non-systemfolder directories.
    /// TODO: Implement non-recursive path scanning.
    /// TODO: Implement exclude list for path adding.
    #[roxygen]
    pub fn add_path(
        &mut self,
        /// The path you want to add to the MSI. This currently must be a directory.
        ///
        /// TODO: Add support for filenames in the future.
        path: &Utf8PathBuf,
        /// The base path/location of where the files and directories from the given path are to be
        /// moved. This must be a Microsoft defined property so arbitrary paths are not allowed.
        destination_base: SystemFolder,
        /// Path to append to the base path in order to get the full path. This is the full path
        /// where the files are to be placed upon install.
        destination_suffix: Option<String>,
    ) -> Result<()> {
        let directory = MsiDirectory::new(&destination_base.to_string(), path)?;
        let directories = directory.flatten_directories();
        DirectoryTable::add(&mut self.package.borrow_mut(), &directories)?;
        Ok(())
    }

    /// Returns the in-memory database that was created by the commands passed to the builder.
    pub fn finish(self) -> Cursor<Vec<u8>> {
        let bare_package = match Rc::try_unwrap(self.package) {
            Ok(pkg) => pkg,
            Err(arc) => panic!("Package still has other references"),
        };

        bare_package.into_inner().into_inner().unwrap()
    }

    /// Write the in-memory MSI data to the output location
    pub fn write(self, output_path: &Utf8PathBuf) -> Result<()> {
        let cursor = self.finish();
        let mut file = File::create(output_path)
            .context(format!("Open output path {output_path} for writing"))?;

        file.write_all(cursor.get_ref())
            .context(format!("Write MSI data to location {output_path}"))?;
        info!("Wrote MSI to {}", output_path);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use assert_fs::prelude::*;
    use assert_fs::{NamedTempFile, TempDir};
    use assertables::*;
    use camino::Utf8Path;
    use itertools::Itertools;
    use msi::{Package, Row, Select};

    use crate::enums::system_folder::SystemFolder;

    use super::MsiBuilder;

    fn setup_test_directory() -> Result<TempDir, Box<dyn std::error::Error>> {
        let root = TempDir::with_prefix("root-")?;
        let dir_1 = root.child("child1");
        dir_1.create_dir_all()?;
        let dir_2 = root.child("child2");
        dir_2.create_dir_all()?;
        let file_1 = root.child("file_1.txt");
        file_1.touch()?;
        let file_2 = dir_1.child("file2.pdf");
        file_2.touch()?;
        Ok(root)
    }
}
