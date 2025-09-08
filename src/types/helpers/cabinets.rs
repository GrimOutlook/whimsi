use crate::tables::builder_list::MsiBuilderList;
use crate::tables::media::cabinet_identifier::{
    CabinetIdGenerator, CabinetIdentifier,
};
use anyhow::ensure;

use crate::types::column::identifier::Identifier;
use crate::types::helpers::cabinet_info::CabinetInfo;
use crate::{
    define_identifier_generator, define_specific_identifier,
    implement_id_generator_for_table, implement_new_for_id_generator_table,
    msi_list_boilerplate,
};

#[derive(Debug)]
pub(crate) struct Cabinets {
    entries: Vec<CabinetInfo>,
    generator: CabinetIdGenerator,
}

impl Cabinets {
    pub fn add_new(&mut self, id: CabinetIdentifier) -> anyhow::Result<()> {
        ensure!(
            !self.has_id(&id),
            format!("Cabinet with ID {} already exists", id)
        );
        let new = CabinetInfo::new(id);
        self.entries.push(new);
        Ok(())
    }

    pub fn has_id(&self, id: &CabinetIdentifier) -> bool {
        self.entries.iter().any(|cab| cab.id() == id)
    }

    pub fn find_id(&self, id: &CabinetIdentifier) -> Option<&CabinetInfo> {
        self.entries.iter().find(|cab| cab.id() == id)
    }

    pub fn find_id_mut(
        &mut self,
        id: &CabinetIdentifier,
    ) -> Option<&mut CabinetInfo> {
        self.entries.iter_mut().find(|cab| cab.id() == id)
    }

    pub fn entries(&self) -> &Vec<CabinetInfo> {
        &self.entries
    }
}

impl MsiBuilderList for Cabinets {
    type ListValue = CabinetInfo;
    msi_list_boilerplate!();
}

implement_new_for_id_generator_table!(Cabinets, CabinetIdGenerator);
implement_id_generator_for_table!(Cabinets, CabinetIdGenerator);
