use std::cell::RefCell;
use std::rc::Rc;

use crate::constants::*;
use crate::define_identifier_generator;
use crate::implement_id_generator_for_table;
use crate::implement_new_for_id_generator_table;
use crate::msi_list_boilerplate;
use crate::msi_table_boilerplate;
use crate::tables::builder_list::MsiBuilderList;
use crate::tables::builder_table::MsiBuilderTable;
use crate::tables::feature::dao::FeatureDao;
use crate::tables::feature::identifier::FeatureIdentifier;
use crate::tables::file::dao::FileDao;
use crate::types::column::identifier::Identifier;
use crate::types::column::identifier::{self};
use crate::types::helpers::id_generator::IdGenerator;

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

impl FeatureTable {
    pub fn new(identifiers: Rc<RefCell<Vec<Identifier>>>) -> Self {
        let entries = vec![FeatureDao::default()];
        let generator = FeatureIdGenerator::from(identifiers);
        Self { entries, generator }
    }
}

msi_list_boilerplate!(FeatureTable, FeatureDao);
implement_id_generator_for_table!(FeatureTable, FeatureIdGenerator);
