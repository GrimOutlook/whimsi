use std::cell::RefCell;
use std::rc::Rc;

use crate::tables::builder_list::MsiBuilderList;
use crate::tables::feature::identifier::FeatureIdentifier;
use crate::types::column::identifier::{self, Identifier};
use crate::types::helpers::id_generator::IdGenerator;
use crate::{
    constants::*,
    define_identifier_generator, msi_table_boilerplate,
    tables::{
        builder_table::MsiBuilderTable, feature::dao::FeatureDao,
        file::dao::FileDao,
    },
};
use crate::{
    implement_id_generator_for_table, implement_new_for_id_generator_table,
    msi_list_boilerplate,
};

define_identifier_generator!(feature);

#[derive(Debug, Clone)]
pub struct FeatureTable {
    entries: Vec<FeatureDao>,
    generator: FeatureIdGenerator,
}

impl FeatureTable {
    pub fn get_default_feature(&self) -> Option<&FeatureDao> {
        self.entries.iter().find(|feature| {
            feature.feature().to_string() == DEFAULT_FEATURE_IDENTIFIER
        })
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

impl FeatureTable {
    pub fn new(identifiers: Rc<RefCell<Vec<Identifier>>>) -> Self {
        let entries = vec![FeatureDao::default()];
        let generator = FeatureIdGenerator::from(identifiers);
        Self { entries, generator }
    }
}

implement_id_generator_for_table!(FeatureTable, FeatureIdGenerator);
