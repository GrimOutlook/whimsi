use anyhow::ensure;

use crate::tables::media::cabinet_identifier::CabinetIdentifier;
use crate::types::column::identifier::Identifier;
use crate::types::helpers::cabinet_info::CabinetInfo;

#[derive(Debug, Default)]
pub(crate) struct Cabinets(Vec<CabinetInfo>);
impl Cabinets {
    pub fn add_new(&mut self, id: Identifier) -> anyhow::Result<()> {
        ensure!(
            !self.has_id(&id),
            format!("Cabinet with ID {} already exists", id)
        );
        let new = CabinetInfo::new(id);
        self.0.push(new);
        Ok(())
    }

    pub fn has_id(&self, id: &Identifier) -> bool {
        self.0.iter().any(|cab| cab.id() == id)
    }

    pub fn find_id(&self, id: &Identifier) -> Option<&CabinetInfo> {
        self.0.iter().find(|cab| cab.id() == id)
    }

    pub fn find_id_mut(&mut self, id: &Identifier) -> Option<&mut CabinetInfo> {
        self.0.iter_mut().find(|cab| cab.id() == id)
    }

    pub fn inner(&self) -> &Vec<CabinetInfo> {
        &self.0
    }
}
