use crate::constants::*;
use crate::define_generator_table;
use crate::implement_id_generator_for_table;
use crate::implement_new_for_id_generator_table;
use crate::msi_list_boilerplate;
use crate::msi_table_boilerplate;
use crate::tables::builder_table::MsiBuilderTable;
use crate::tables::service_control::dao::ServiceControlDao;
use crate::tables::service_control::dao::ServiceControlIdGenerator;

define_generator_table!(
    ServiceControl,
    vec![
        whimsi_msi::Column::build("ServiceControl")
            .primary_key()
            .id_string(DEFAULT_IDENTIFIER_MAX_LEN),
        whimsi_msi::Column::build("Name").text_string(255),
        whimsi_msi::Column::build("Event").int16(),
        whimsi_msi::Column::build("Arguments").nullable().formatted_string(255),
        whimsi_msi::Column::build("Wait").nullable().int16(),
        whimsi_msi::Column::build("Component_")
            .id_string(DEFAULT_IDENTIFIER_MAX_LEN),
    ]
);

msi_list_boilerplate!(ServiceControlTable, ServiceControlDao);
implement_id_generator_for_table!(
    ServiceControlTable,
    ServiceControlIdGenerator
);
implement_new_for_id_generator_table!(
    ServiceControlTable,
    ServiceControlIdGenerator
);
