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
use crate::tables::registry::dao::RegistryDao;

define_specific_identifier!(Registry);
define_specific_identifier_parsing!(Registry);
define_identifier_generator!(Registry);
define_generator_table!(
    Registry,
    vec![
        whimsi_msi::Column::build("Registry")
            .primary_key()
            .id_string(DEFAULT_IDENTIFIER_MAX_LEN),
        whimsi_msi::Column::build("Root").int16(),
        whimsi_msi::Column::build("Key")
            .localizable()
            .category(whimsi_msi::Category::RegPath)
            .string(REGPATH_MAX_LEN),
        whimsi_msi::Column::build("Name")
            .localizable()
            .nullable()
            .formatted_string(REGISTRY_NAME_MAX_LEN),
        whimsi_msi::Column::build("Value")
            .localizable()
            .nullable()
            .formatted_string(REGISTRY_VALUE_MAX_LEN),
        whimsi_msi::Column::build("Component_")
            .id_string(DEFAULT_IDENTIFIER_MAX_LEN),
    ]
);

msi_list_boilerplate!(RegistryTable, RegistryDao);
implement_id_generator_for_table!(RegistryTable, RegistryIdGenerator);
implement_new_for_id_generator_table!(RegistryTable, RegistryIdGenerator);
