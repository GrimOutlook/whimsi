use anyhow::Context;
use getset::Getters;

use crate::int_val;
use crate::opt_str_val;
use crate::str_val;
use crate::tables::dao::IsDao;
use crate::tables::directory::directory_identifier::DirectoryIdentifier;
use crate::types::column::condition::Condition;
use crate::types::column::guid::Guid;
use crate::types::column::identifier::Identifier;

#[derive(Debug, Clone, Getters)]
#[getset(get = "pub(crate)")]
pub struct ComponentDao {
    component: Identifier,
    component_id: Option<Guid>,
    directory: DirectoryIdentifier,
    attributes: i16,
    condition: Option<Condition>,
    key_path: Option<Identifier>,
}

impl ComponentDao {
    pub fn new(
        component_id: Identifier,
        directory_id: DirectoryIdentifier,
    ) -> ComponentDao {
        ComponentDao {
            component: component_id,
            directory: directory_id,
            component_id: None,
            attributes: 0,
            condition: None,
            key_path: None,
        }
    }

    pub fn with_keypath(mut self, key_path: Identifier) -> Self {
        self.key_path = Some(key_path);
        self
    }
}

impl IsDao for ComponentDao {
    fn to_row(&self) -> Vec<msi::Value> {
        vec![
            str_val!(self.component.to_string()),
            opt_str_val!(self.component_id),
            str_val!(self.directory),
            int_val!(self.attributes),
            opt_str_val!(self.condition),
            opt_str_val!(self.key_path),
        ]
    }

    fn conflicts(&self, other: &Self) -> bool {
        self.component == other.component
    }
}
