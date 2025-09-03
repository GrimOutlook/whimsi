use itertools::Itertools;

use crate::{constants::*, msitable_boilerplate, tables::builder_table::MsiBuilderTable};

use super::dao::ComponentDao;

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
}
