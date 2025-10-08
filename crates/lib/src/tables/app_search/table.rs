use crate::{
    constants::DEFAULT_IDENTIFIER_MAX_LEN,
    msi_list_boilerplate, msi_table_boilerplate,
    tables::{app_search::dao::AppSearchDao, builder_table::MsiBuilderTable},
};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct AppSearchTable {
    entries: Vec<AppSearchDao>,
}

msi_list_boilerplate!(AppSearchTable, AppSearchDao);

impl MsiBuilderTable for AppSearchTable {
    type TableValue = AppSearchDao;
    msi_table_boilerplate!();

    fn name(&self) -> &'static str {
        "AppSearch"
    }

    fn columns(&self) -> Vec<msi::Column> {
        vec![
            msi::Column::build("Property")
                .primary_key()
                .id_string(DEFAULT_IDENTIFIER_MAX_LEN),
            msi::Column::build("Signature_")
                .primary_key()
                .id_string(DEFAULT_IDENTIFIER_MAX_LEN),
        ]
    }
}
