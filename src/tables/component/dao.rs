use anyhow::Context;
use getset::Getters;

use crate::int_val;
use crate::opt_str_val;
use crate::str_val;
use crate::types::column::condition::Condition;
use crate::types::column::guid::Guid;
use crate::types::column::identifier::Identifier;

#[derive(Debug, Clone, Getters)]
#[getset(get = "pub(crate)")]
pub struct ComponentDao {
    component: Identifier,
    component_id: Option<Guid>,
    directory: Identifier,
    attributes: i16,
    condition: Option<Condition>,
    key_path: Option<Identifier>,
}

impl ComponentDao {
    pub fn to_row(&self) -> Vec<msi::Value> {
        vec![
            str_val!(self.component.to_string()),
            opt_str_val!(self.component_id),
            str_val!(self.directory),
            int_val!(self.attributes),
            opt_str_val!(self.condition),
            opt_str_val!(self.key_path),
        ]
    }
}
