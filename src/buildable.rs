use std::{alloc::System, str::FromStr};

use getset::Getters;
use itertools::Itertools;
use rand::distr::{Alphanumeric, SampleString};
use tracing::{debug, info};

use crate::{
    builder::MsiBuilder,
    constants::IDENTIFIER_MAX_LEN,
    tables::{
        MsiBuilderTables,
        builder_table::MsiBuilderTable,
        directory::{dao::DirectoryDao, kind::DirectoryKind, system_directory::SystemDirectory},
        meta::MetaInformation,
    },
    types::column::{default_dir::DefaultDir, identifier::Identifier},
};

type Identifiers = Vec<Identifier>;

#[derive(Debug, Default, Getters)]
#[getset(get = "pub")]
pub struct MsiBuildable {
    /// Tracks identifiers used to relate items between tables.
    pub(crate) identifiers: Identifiers,
    pub(crate) tables: MsiBuilderTables,
    pub(crate) meta: MetaInformation,
}

impl MsiBuildable {
    pub(crate) fn new() -> MsiBuildable {
        Self::default()
    }

    /// Build the MSI from all information given to MSIBuilder.
    pub fn build<F: std::io::Read + std::io::Write + std::io::Seek>(
        self,
        container: F,
    ) -> anyhow::Result<msi::Package<F>> {
        info!("Building MSI");
        let mut package = msi::Package::create(*self.meta.package_type(), container)?;
        self.tables.write_to_package(&mut package)?;
        Ok(package)
    }

    /// Generate an `Identifier` not already listed in the `Identifiers` list. Returns the
    /// generated `Identifier` to the user for use and adds a clone to the `Identifiers` list so it
    /// cannot be generated and used again.
    ///
    /// # Panics
    /// If a randomly generated ID is not a valid `Identifier`. Should not be possible given the
    /// random string characterset and parsing rules of an `Identifier`.
    pub fn generate_id(&mut self) -> Identifier {
        loop {
            // Start the generated ID string with an underscore since identifiers aren't allowed to
            // start with a number and this is the simplest way around rerolling if the randomly
            // generated identifier starts with a number.
            let mut id_string = "_".to_string();
            let id_string_len = id_string.len();
            Alphanumeric.append_string(
                &mut rand::rng(),
                &mut id_string,
                IDENTIFIER_MAX_LEN - id_string_len,
            );
            let id = Identifier::from_str(&id_string).unwrap();
            if !self.has_identifier(&id) {
                self.identifiers.push(id.clone());
                return id;
            }
        }
    }

    pub fn has_identifier(&self, identifier: &Identifier) -> bool {
        self.identifiers.iter().any(|i| i == identifier)
    }

    pub fn with_directories(
        mut self,
        system_directories: Vec<SystemDirectory>,
    ) -> anyhow::Result<Self> {
        debug!("Generating DirectoryTable entries from given directories");

        // Add the DAOs for the system directories and it's direct contents.
        let mut daos = Vec::new();
        for system_directory in system_directories {
            let system_folder = system_directory.system_folder();
            // Add the system directory DAO
            daos.push(DirectoryDao::from(system_folder));
            daos.append(
                &mut self
                    .nested_directory_daos(&system_directory, &Identifier::from(system_folder)),
            )
        }

        self.tables.directory_mut().add_all(daos);

        Ok(self)
    }

    fn nested_directory_daos(
        &mut self,
        directory: &impl DirectoryKind,
        parent_id: &Identifier,
    ) -> Vec<DirectoryDao> {
        let mut daos = Vec::new();
        for dir in directory.contained_directories().into_iter() {
            let id = self.generate_id();
            daos.push(DirectoryDao::new(
                DefaultDir::from(dir.name().clone()),
                id.clone(),
                parent_id.clone(),
            ));
            daos.append(&mut self.nested_directory_daos(dir, &id));
        }
        daos
    }
}
