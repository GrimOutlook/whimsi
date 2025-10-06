use crate::{
    constants::*,
    msi_list_boilerplate, msi_table_boilerplate,
    tables::{
        builder_table::MsiBuilderTable, service_control::dao::ServiceControlDao,
    },
};

#[derive(Debug, Default)]
pub struct ServiceControlTable {
    entries: Vec<ServiceControlDao>,
}

impl MsiBuilderTable for ServiceControlTable {
    type TableValue = ServiceControlDao;

    msi_table_boilerplate!();

    fn name(&self) -> &'static str {
        "ServiceControl"
    }

    fn columns(&self) -> Vec<whimsi_msi::Column> {
        vec![
            whimsi_msi::Column::build("ServiceControl")
                .primary_key()
                .id_string(DEFAULT_IDENTIFIER_MAX_LEN),
            whimsi_msi::Column::build("Name").text_string(255),
            whimsi_msi::Column::build("Event").int16(),
            whimsi_msi::Column::build("Arguments")
                .nullable()
                .formatted_string(255),
            whimsi_msi::Column::build("Wait").nullable().int16(),
            whimsi_msi::Column::build("Component_")
                .id_string(DEFAULT_IDENTIFIER_MAX_LEN),
        ]
    }
}

msi_list_boilerplate!(ServiceControlTable, ServiceControlDao);
