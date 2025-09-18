use crate::{
    constants::CONDITION_MAX_LEN,
    msi_list_boilerplate, msi_table_boilerplate,
    tables::{
        builder_table::MsiBuilderTable,
        launch_condition::dao::LaunchConditionDao,
    },
};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct LaunchConditionTable {
    entries: Vec<LaunchConditionDao>,
}

msi_list_boilerplate!(LaunchConditionTable, LaunchConditionDao);

impl MsiBuilderTable for LaunchConditionTable {
    type TableValue = LaunchConditionDao;
    msi_table_boilerplate!();

    fn name(&self) -> &'static str {
        "LaunchCondition"
    }

    fn columns(&self) -> Vec<whimsi_msi::Column> {
        vec![
            whimsi_msi::Column::build("Condition")
                .primary_key()
                .category(whimsi_msi::Category::Condition)
                .string(CONDITION_MAX_LEN),
            // TODO: Make this a proper constant.
            whimsi_msi::Column::build("Description")
                .localizable()
                .formatted_string(255),
        ]
    }
}
