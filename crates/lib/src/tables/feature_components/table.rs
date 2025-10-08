use crate::constants::*;
use crate::msi_list_boilerplate;
use crate::msi_table_boilerplate;
use crate::tables::builder_list::MsiBuilderList;
use crate::tables::builder_table::MsiBuilderTable;
use crate::tables::feature_components::dao::FeatureComponentsDao;

#[derive(Debug, Clone, Default)]
pub struct FeatureComponentsTable {
    entries: Vec<FeatureComponentsDao>,
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
                .id_string(FEATURE_IDENTIFIER_MAX_LEN),
            msi::Column::build("Component_")
                .primary_key()
                .id_string(DEFAULT_IDENTIFIER_MAX_LEN),
        ]
    }
}

msi_list_boilerplate!(FeatureComponentsTable, FeatureComponentsDao);
