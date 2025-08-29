use anyhow::Context;

use crate::{
    tables::file::helper::File,
    types::column::{condition::Condition, guid::Guid, identifier::Identifier},
};

#[derive(Debug, Clone)]
pub struct ComponentDao {
    component: Identifier,
    component_id: Option<Guid>,
    directory: Identifier,
    attributes: u32,
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
        let attributes = component.attributes().bits();
        Self {
            component: component_id,
            key_path: Some(file_id.clone()),
            directory: directory_id.clone(),
            attributes: attributes
                .try_into()
                .context(format!(
                    "Attributes value [{}] for file [{}] component is too large",
                    attributes, file
                ))
                .unwrap(),
            component_id: component.guid().map(|uuid| uuid.into()),
            condition: component.condition().clone(),
        }
    }
}
