use anyhow::Context;
use getset::Getters;

use crate::{
    int_val, opt_str_val, str_val,
    tables::file::helper::File,
    types::column::{condition::Condition, guid::Guid, identifier::Identifier},
};

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
    pub fn from_file(
        component_id: Identifier,
        file: &File,
        file_id: &Identifier,
        directory_id: &Identifier,
    ) -> Self {
        let component = file.component().clone();
        Self {
            component: component_id,
            key_path: Some(file_id.clone()),
            directory: directory_id.clone(),
            attributes: component.attributes().clone(),
            component_id: component.guid().map(|uuid| uuid.into()),
            condition: component.condition().clone(),
        }
    }

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
