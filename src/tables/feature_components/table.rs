use crate::{
    constants::DEFAULT_IDENTIFIER_MAX_LEN,
    msi_list_boilerplate, msi_table_boilerplate,
    tables::{
        builder_list::MsiBuilderList, builder_table::MsiBuilderTable,
        feature_components::dao::FeatureComponentsDao,
    },
};

#[derive(Debug, Clone, Default)]
pub struct FeatureComponentsTable {
    entries: Vec<FeatureComponentsDao>,
}

impl MsiBuilderList for FeatureComponentsTable {
    type ListValue = FeatureComponentsDao;
    msi_list_boilerplate!();
}

impl MsiBuilderTable for FeatureComponentsTable {
    type TableValue = FeatureComponentsDao;

    msi_table_boilerplate!();

    fn name(&self) -> &'static str {
        "FeatureComponents"
    }

    fn columns(&self) -> Vec<msi::Column> {
        vec![
            msi::Column::build("Feature_")
                .primary_key()
                .id_string(DEFAULT_IDENTIFIER_MAX_LEN),
            msi::Column::build("Component_")
                .primary_key()
                .id_string(DEFAULT_IDENTIFIER_MAX_LEN),
        ]
    }
}
