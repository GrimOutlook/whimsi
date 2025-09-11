use anyhow::ensure;
use itertools::Itertools;

use super::dao::ComponentDao;
use crate::constants::*;
use crate::define_generator_table;
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

define_specific_identifier!(Component);
define_specific_identifier_parsing!(Component);
define_identifier_generator!(Component);
define_generator_table!(
    Component,
    vec![
        whimsi_msi::Column::build("Component")
            .primary_key()
            .id_string(DEFAULT_IDENTIFIER_MAX_LEN),
        whimsi_msi::Column::build("ComponentId")
            .nullable()
            .category(whimsi_msi::Category::Guid)
            .string(GUID_MAX_LEN),
        whimsi_msi::Column::build("Directory_").id_string(DEFAULT_IDENTIFIER_MAX_LEN),
        whimsi_msi::Column::build("Attributes").int16(),
        whimsi_msi::Column::build("Condition")
            .nullable()
            .category(whimsi_msi::Category::Condition)
            .string(CONDITION_MAX_LEN),
        whimsi_msi::Column::build("KeyPath")
            .nullable()
            .id_string(DEFAULT_IDENTIFIER_MAX_LEN),
    ]
);

msi_list_boilerplate!(ComponentTable, ComponentDao);
implement_id_generator_for_table!(ComponentTable, ComponentIdGenerator);
implement_new_for_id_generator_table!(ComponentTable, ComponentIdGenerator);
