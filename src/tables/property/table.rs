use std::collections::HashMap;

use itertools::Itertools;

use crate::types::column::identifier::Identifier;

#[derive(Clone, Debug, Default)]
pub struct PropertyTable(HashMap<Identifier, String>);

impl PropertyTable {
    pub fn has_property(&self, id: &Identifier) -> bool {
        self.0.keys().contains(id)
    }
}
