use crate::{
    constants::DEFAULT_IDENTIFIER_MAX_LEN,
    msi_list_boilerplate, msi_table_boilerplate,
    tables::{
        builder_table::MsiBuilderTable, custom_action::dao::CustomActionDao,
    },
};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct CustomActionTable {
    entries: Vec<CustomActionDao>,
}

msi_list_boilerplate!(CustomActionTable, CustomActionDao);

impl MsiBuilderTable for CustomActionTable {
    type TableValue = CustomActionDao;
    msi_table_boilerplate!();

    fn name(&self) -> &'static str {
        "CustomAction"
    }

    fn columns(&self) -> Vec<whimsi_msi::Column> {
        vec![
            whimsi_msi::Column::build("Action")
                .primary_key()
                .id_string(DEFAULT_IDENTIFIER_MAX_LEN),
            whimsi_msi::Column::build("Type").int16(),
            // TODO: Make constant
            whimsi_msi::Column::build("Source")
                .nullable()
                .category(whimsi_msi::Category::CustomSource)
                .string(DEFAULT_IDENTIFIER_MAX_LEN),
            whimsi_msi::Column::build("Target")
                .nullable()
                .formatted_string(255),
            whimsi_msi::Column::build("ExtendedType").nullable().int32(),
        ]
    }
}
