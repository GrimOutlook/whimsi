use std::str::FromStr;

use getset::Getters;
use rand::distr::{Alphanumeric, SampleString};
use tracing::info;

use crate::{
    tables::{MsiBuilderTables, meta::MetaInformation},
    types::column::identifier::Identifier,
};

type Identifiers = Vec<Identifier>;

#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct MsiBuildable {
    /// Tracks identifiers used to relate items between tables.
    #[getset(get_mut = "pub(crate)")]
    identifiers: Identifiers,
    tables: MsiBuilderTables,
    meta: MetaInformation,
}

impl MsiBuildable {
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

    /// Generate an `Identifier` not already in the `Identifiers` hashmap.
    /// Identifier is 72 characters to reduce likelihood of collision. 72 Character limit is taken
    /// from Directory table column max char length.
    fn generate_id(&self) -> Identifier {
        loop {
            let mut id_string = "_".to_string();
            Alphanumeric.append_string(&mut rand::rng(), &mut id_string, 71);
            let id = Identifier::from_str(&id_string).unwrap();
            if !self.has_identifier(&id) {
                return id;
            }
        }
    }

    pub fn has_identifier(&self, identifier: &Identifier) -> bool {
        self.identifiers.iter().any(|i| i == identifier)
    }
}
