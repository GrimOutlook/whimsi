use itertools::Itertools;

use crate::constants::*;
use crate::msi_list_boilerplate;
use crate::msi_table_boilerplate;
use crate::tables::advt_execute_sequence::dao::AdvtExecuteSequenceDao;
use crate::tables::builder_list::MsiBuilderList;
use crate::tables::builder_table::MsiBuilderTable;
use crate::types::standard_action::AdvtAction;

/// The AdvtExecuteSequence table lists actions the installer calls when the
/// top-level ADVERTISE action is executed.
#[derive(Debug, Clone)]
pub struct AdvtExecuteSequenceTable {
    entries: Vec<AdvtExecuteSequenceDao>,
}

msi_list_boilerplate!(AdvtExecuteSequenceTable, AdvtExecuteSequenceDao);

impl MsiBuilderTable for AdvtExecuteSequenceTable {
    type TableValue = AdvtExecuteSequenceDao;

    msi_table_boilerplate!();

    fn name(&self) -> &'static str {
        "AdvtExecuteSequence"
    }

    fn columns(&self) -> Vec<whimsi_msi::Column> {
        vec![
            whimsi_msi::Column::build("Action")
                .primary_key()
                .id_string(DEFAULT_IDENTIFIER_MAX_LEN),
            whimsi_msi::Column::build("Condition")
                .nullable()
                .category(whimsi_msi::Category::Condition)
                .string(CONDITION_MAX_LEN),
            whimsi_msi::Column::build("Sequence").nullable().int16(),
        ]
    }
}

impl Default for AdvtExecuteSequenceTable {
    fn default() -> Self {
        let entries = Vec::from(ADVT_EXECUTE_SEQUENCE_DEFAULT_ACTIONS)
            .into_iter()
            .map_into()
            .collect_vec();
        Self { entries }
    }
}
