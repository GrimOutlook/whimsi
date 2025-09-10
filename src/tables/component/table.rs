use anyhow::ensure;
use itertools::Itertools;

use super::dao::ComponentDao;
use crate::constants::*;
use crate::define_identifier_generator;
use crate::define_specific_identifier;
use crate::define_specific_identifier_parsing;
use crate::implement_id_generator_for_table;
use crate::implement_new_for_id_generator_table;
use crate::msi_list_boilerplate;
use crate::msi_table_boilerplate;
use crate::tables::builder_list::MsiBuilderList;
use crate::tables::builder_table::MsiBuilderTable;
use crate::tables::dao::IsDao;
use crate::types::column::identifier::Identifier;
use crate::types::column::identifier::ToIdentifier;

define_specific_identifier!(component);
define_specific_identifier_parsing!(component);
define_identifier_generator!(component);

#[derive(Debug, Clone)]
pub struct ComponentTable {
    entries: Vec<ComponentDao>,
    generator: ComponentIdGenerator,
}

impl MsiBuilderTable for ComponentTable {
    type TableValue = ComponentDao;

    // Boilderplate needed to access information on the inner object
    msi_table_boilerplate!();

    fn name(&self) -> &'static str {
        "Component"
    }

    fn columns(&self) -> Vec<msi::Column> {
        vec![
            msi::Column::build("Component")
                .primary_key()
                .id_string(DEFAULT_IDENTIFIER_MAX_LEN),
            msi::Column::build("ComponentId")
                .nullable()
                .category(msi::Category::Guid)
                .string(GUID_MAX_LEN),
            msi::Column::build("Directory_")
                .id_string(DEFAULT_IDENTIFIER_MAX_LEN),
            msi::Column::build("Attributes").int16(),
            msi::Column::build("Condition")
                .nullable()
                .category(msi::Category::Condition)
                .string(CONDITION_MAX_LEN),
            msi::Column::build("KeyPath")
                .nullable()
                .id_string(DEFAULT_IDENTIFIER_MAX_LEN),
        ]
    }
}

impl MsiBuilderList for ComponentTable {
    type ListValue = ComponentDao;

    msi_list_boilerplate!();
}

implement_id_generator_for_table!(ComponentTable, ComponentIdGenerator);
implement_new_for_id_generator_table!(ComponentTable, ComponentIdGenerator);
