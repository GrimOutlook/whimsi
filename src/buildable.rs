use std::{alloc::System, str::FromStr};

use getset::{Getters, WithSetters};
use itertools::Itertools;
use rand::distr::{Alphanumeric, SampleString};
use tracing::{debug, info, trace};

use crate::{
    builder::MsiBuilder,
    constants::IDENTIFIER_MAX_LEN,
    tables::{
        MsiBuilderTables,
        builder_table::MsiBuilderTable,
        directory::{
            dao::DirectoryDao, kind::DirectoryKind, system_directory::SystemDirectory,
            traits::container::Container,
        },
        file::helper::File,
        meta::MetaInformation,
    },
    types::{
        column::{
            default_dir::DefaultDir,
            identifier::{Identifier, ToIdentifier},
        },
        helpers::directory_item::DirectoryItem,
        properties::system_folder::SystemFolder,
    },
};

type Identifiers = Vec<Identifier>;

#[derive(Debug, Default, Getters, WithSetters)]
#[getset(get = "pub", set_with)]
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
                // Ensure that the identifier cannot be generated again.
                self.identifiers.push(id.clone());
                return id;
            }
        }
    }

    pub fn has_identifier(&self, identifier: &Identifier) -> bool {
        self.identifiers.iter().any(|i| i == identifier)
    }

    fn add_to_directory_table(
        &mut self,
        directory: DirectoryKind,
        parent: Identifier,
    ) -> anyhow::Result<Identifier> {
        let id: Identifier;
        let dao = match directory {
            DirectoryKind::Directory(directory) => {
                id = self.generate_id();
                DirectoryDao::new(
                    DefaultDir::from(directory.name().clone()),
                    id.clone(),
                    parent,
                )
            }
            DirectoryKind::SystemDirectory(system_directory) => {
                let folder = system_directory.system_folder();
                id = folder.to_identifier();
                // Extra check the verify that the system folder is only specified once.
                self.identifiers.push(id.clone());
                // The parent passed in for system_folders is always the same so I've simplified it
                // to be a From conversion.
                DirectoryDao::from(folder)
            }
        };
        trace!("Added directory of ID [{id}] to DirectoryTable");
        self.tables.directory_mut().add(dao);

        Ok(id)
    }

    fn add_directory_recursively_to_tables(
        &mut self,
        parent_id: &Identifier,
        directory: &DirectoryKind,
    ) -> anyhow::Result<()> {
        let directory_id =
            self.add_to_directory_table(DirectoryKind::from(directory.clone()), parent_id.clone())?;
        let component_id = self.add_to_component_table(directory_id, directory.component());

        for item in contents {
            match item {
                DirectoryItem::File(file) => self.add_file_info_to_tables(&file, &directory_id)?,
                DirectoryItem::Directory(directory) => {
                    let id = self.add_to_directory_table(
                        // TODO: I don't like cloning this directory, we should be able to take
                        // ownership of it and move it.
                        DirectoryKind::Directory(directory.clone()),
                        directory_id.clone(),
                    )?;
                    self.add_contents_to_tables(&id, directory.contents())?
                }
            }
        }
        Ok(())
    }

    fn add_file_info_to_tables(
        &mut self,
        file: &File,
        directory_id: &Identifier,
    ) -> anyhow::Result<()> {
        todo!()
    }
}

impl TryFrom<MsiBuilder> for MsiBuildable {
    type Error = anyhow::Error;
    fn try_from(value: MsiBuilder) -> anyhow::Result<MsiBuildable> {
        debug!("Creating Buildable from Builder");
        let mut buildable = MsiBuildable::new().with_meta(value.meta().clone());
        // System directories have to be handled in a special way since they only have identifiers
        // and have a known parent.
        for directory in value.system_directories() {
            let directory_id = buildable.add_to_directory_table(
                DirectoryKind::from(directory.clone()),
                SystemFolder::TARGETDIR.to_identifier(),
            )?;

            buildable.add_contents_to_tables(&directory_id, directory.contents())?
        }

        todo!("Finish converting to buildable")
    }
}
