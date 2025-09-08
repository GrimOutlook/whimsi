use crate::{
    constants::*,
    msitable_boilerplate,
    tables::{
        builder_table::MsiBuilderTable, feature::dao::FeatureDao,
        file::dao::FileDao,
    },
};

#[derive(Debug, Clone, Default)]
pub struct FeatureTable(Vec<FeatureDao>);
impl FeatureTable {
    pub fn get_default_feature(&self) -> &FeatureDao {
        todo!()
    }
}
impl MsiBuilderTable for FeatureTable {
    type TableValue = FeatureDao;

    // Boilderplate needed to access information on the inner object
    msitable_boilerplate!();

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
                .id_string(IDENTIFIER_MAX_LEN),
            msi::Column::build("Attributes").int16(),
        ]
    }
}
