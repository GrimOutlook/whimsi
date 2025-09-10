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
use crate::tables::registry::dao::RegistryDao;

define_specific_identifier!(registry);
define_specific_identifier_parsing!(registry);
define_identifier_generator!(registry);

#[derive(Debug, Clone, PartialEq)]
pub struct RegistryTable {
    entries: Vec<RegistryDao>,
    generator: RegistryIdGenerator,
}

impl MsiBuilderList for RegistryTable {
    type ListValue = RegistryDao;

    msi_list_boilerplate!();
}

impl MsiBuilderTable for RegistryTable {
    type TableValue = RegistryDao;

    msi_table_boilerplate!();

    fn name(&self) -> &'static str {
        "Registry"
    }

    fn columns(&self) -> Vec<msi::Column> {
        vec![
            msi::Column::build("Registry")
                .primary_key()
                .id_string(DEFAULT_IDENTIFIER_MAX_LEN),
            msi::Column::build("Root").int16(),
            msi::Column::build("Key")
                .category(msi::Category::RegPath)
                .string(REGPATH_MAX_LEN),
            msi::Column::build("Name")
                .nullable()
                .formatted_string(REGISTRY_NAME_MAX_LEN),
            msi::Column::build("Value")
                .nullable()
                .formatted_string(REGISTRY_VALUE_MAX_LEN),
            msi::Column::build("Component_")
                .id_string(DEFAULT_IDENTIFIER_MAX_LEN),
        ]
    }
}

implement_id_generator_for_table!(RegistryTable, RegistryIdGenerator);
implement_new_for_id_generator_table!(RegistryTable, RegistryIdGenerator);
