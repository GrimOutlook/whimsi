use itertools::Itertools;

use crate::{
    constants::*,
    msi_list_boilerplate, msi_table_boilerplate,
    tables::{
        builder_list::MsiBuilderList, builder_table::MsiBuilderTable,
        generic_sequence::dao::GenericSequenceDao,
    },
};

#[derive(Debug, Clone)]
pub struct InstallUiSequenceTable {
    entries: Vec<GenericSequenceDao>,
}

msi_list_boilerplate!(InstallUiSequenceTable, GenericSequenceDao);

impl MsiBuilderTable for InstallUiSequenceTable {
    type TableValue = GenericSequenceDao;
    msi_table_boilerplate!();

    fn name(&self) -> &'static str {
        "InstallUISequence"
    }

    fn columns(&self) -> Vec<msi::Column> {
        vec![
            msi::Column::build("Action")
                .primary_key()
                .id_string(DEFAULT_IDENTIFIER_MAX_LEN),
            msi::Column::build("Condition")
                .nullable()
                .category(msi::Category::Condition)
                .string(CONDITION_MAX_LEN),
            msi::Column::build("Sequence").nullable().int16(),
        ]
    }
}

impl Default for InstallUiSequenceTable {
    fn default() -> Self {
        let entries = Vec::from(INSTALL_UI_SEQUENCE_DEFAULT_ACTIONS)
            .into_iter()
            .map_into()
            .collect_vec();
        Self { entries }
    }
}
