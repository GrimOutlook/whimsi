use anyhow::ensure;

use crate::types::column::identifier::{Identifier, ToIdentifier};
use crate::types::helpers::cabinet_info::CabinetInfo;
use crate::types::helpers::id_generator::IdentifierGenerator;

// TODO: This area could be almost completely removed by making the MsiTables derive macro more
// modular.
//
// -- Begin section that could be basically removed by an altered derive macro
struct CabinetIdentifier(Identifier);
impl ToIdentifier for CabinetIdentifier {
    fn to_identifier(&self) -> Identifier {
        self.0
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) struct CabinetIdentifierGenerator {
    count: usize,
    // A reference to a vec of all used Identifiers that should not be generated again.
    // These are all identifiers that inhabit a primary_key column.
    used: std::rc::Rc<std::cell::RefCell<Vec<Identifier>>>,
}

impl IdentifierGenerator for CabinetIdentifierGenerator {
    type IdentifierType = CabinetIdentifier;

    fn id_prefix(&self) -> &str {
        "CABINET"
    }

    fn used(&self) -> &std::rc::Rc<std::cell::RefCell<Vec<Identifier>>> {
        &self.used
    }

    fn count(&self) -> usize {
        self.count
    }

    fn count_mut(&mut self) -> &mut usize {
        &mut self.count
    }
}

impl From<std::rc::Rc<std::cell::RefCell<Vec<Identifier>>>>
    for CabinetIdentifierGenerator
{
    fn from(value: std::rc::Rc<std::cell::RefCell<Vec<Identifier>>>) -> Self {
        let count = value.borrow().len();
        Self { used: value, count: 0 }
    }
}
impl std::str::FromStr for CabinetIdentifier {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        Ok(Self(Identifier::from_str(s)?))
    }
}
// -- End section that could be removed with derive macro changes -------------

#[derive(Debug)]
pub(crate) struct Cabinets {
    entries: Vec<CabinetInfo>,
    generator: CabinetIdentifierGenerator,
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
