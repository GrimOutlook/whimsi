use anyhow::ensure;
use itertools::Itertools;

use super::dao::ComponentDao;
use crate::constants::*;
use crate::msitable_boilerplate;
use crate::tables::builder_table::MsiBuilderTable;

#[derive(Debug, Clone, Default)]
pub struct ComponentTable(Vec<ComponentDao>);
impl MsiBuilderTable for ComponentTable {
    type TableValue = ComponentDao;

    // Boilderplate needed to access information on the inner object
    msitable_boilerplate!();

    fn name(&self) -> &'static str {
        "Component"
    }

    fn columns(&self) -> Vec<msi::Column> {
        vec![
            msi::Column::build("Component")
                .primary_key()
                .id_string(IDENTIFIER_MAX_LEN),
            msi::Column::build("ComponentId")
                .nullable()
                .category(msi::Category::Guid)
                .string(GUID_MAX_LEN),
            msi::Column::build("Directory_").id_string(IDENTIFIER_MAX_LEN),
            msi::Column::build("Attributes").int16(),
            msi::Column::build("Condition")
                .nullable()
                .category(msi::Category::Condition)
                .string(CONDITION_MAX_LEN),
            msi::Column::build("KeyPath")
                .nullable()
                .id_string(IDENTIFIER_MAX_LEN),
        ]
    }

    fn rows(&self) -> Vec<Vec<msi::Value>> {
        self.values().iter().map(ComponentDao::to_row).collect_vec()
    }

    fn contains(&self, dao: &ComponentDao) -> bool {
        self.0
            .iter()
            .find(|entry| entry.component() == dao.component())
            .is_some()
    }

    fn add(&mut self, dao: Self::TableValue) -> anyhow::Result<()> {
        // TODO: Create actual error for component ID collision.
        ensure!(!self.contains(&dao), "TEMPERROR");
        self.0.push(dao);
        Ok(())
    }
}
