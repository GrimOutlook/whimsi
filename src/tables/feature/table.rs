use crate::msi_list_boilerplate;
use crate::tables::builder_list::MsiBuilderList;
use crate::tables::feature::identifier::FeatureIdentifier;
use crate::types::helpers::id_generator::IdGenerator;
use crate::{
    constants::*,
    define_identifier_generator, msi_table_boilerplate,
    tables::{
        builder_table::MsiBuilderTable, feature::dao::FeatureDao,
        file::dao::FileDao,
    },
};

define_identifier_generator!(feature);

#[derive(Debug, Clone)]
pub struct FeatureTable {
    entries: Vec<FeatureDao>,
    id_generator: FeatureIdGenerator,
}

impl FeatureTable {
    pub fn get_default_feature(&self) -> &FeatureDao {
        todo!()
    }
}

impl MsiBuilderTable for FeatureTable {
    type TableValue = FeatureDao;

    // Boilderplate needed to access information on the inner object
    msi_table_boilerplate!();

    fn name(&self) -> &'static str {
        "Feature"
    }

    fn columns(&self) -> Vec<msi::Column> {
        vec![
            msi::Column::build("Feature")
                .primary_key()
                .id_string(FEATURE_IDENTIFIER_MAX_LEN),
            msi::Column::build("Feature_Parent")
                .nullable()
                .id_string(FEATURE_IDENTIFIER_MAX_LEN),
            msi::Column::build("Title").nullable().string(TITLE_MAX_LEN),
            msi::Column::build("Description")
                .nullable()
                .string(DESCRIPTION_MAX_LEN),
            msi::Column::build("Display").nullable().int16(),
            msi::Column::build("Level").int16(),
            msi::Column::build("Directory")
                .nullable()
                .id_string(DEFAULT_IDENTIFIER_MAX_LEN),
            msi::Column::build("Attributes").int16(),
        ]
    }
}

impl MsiBuilderList for FeatureTable {
    type ListValue = FeatureDao;

    msi_list_boilerplate!();
}
