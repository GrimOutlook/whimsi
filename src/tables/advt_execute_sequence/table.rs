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

impl MsiBuilderList for AdvtExecuteSequenceTable {
    type ListValue = AdvtExecuteSequenceDao;

    msi_list_boilerplate!();
}

impl MsiBuilderTable for AdvtExecuteSequenceTable {
    type TableValue = AdvtExecuteSequenceDao;

    msi_table_boilerplate!();

    fn name(&self) -> &'static str {
        "AdvtExecuteSequence"
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

impl Default for AdvtExecuteSequenceTable {
    fn default() -> Self {
        let entries = vec![
            AdvtAction::CostInitialize.into(),
            AdvtAction::CostFinalize.into(),
            AdvtAction::InstallValidate.into(),
            AdvtAction::InstallInitialize.into(),
            AdvtAction::InstallFinalize.into(),
            AdvtAction::PublishFeatures.into(),
            AdvtAction::PublishProduct.into(),
        ];
        Self { entries }
    }
}
