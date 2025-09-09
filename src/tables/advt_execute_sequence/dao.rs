use crate::{
    constants::{self, *},
    int_val, opt_int_val, opt_str_val, str_val,
    tables::{builder_list_entry::MsiBuilderListEntry, dao::IsDao},
    types::{
        column::{
            condition::Condition,
            identifier::{Identifier, ToOptionalIdentifier},
            sequence::Sequence,
        },
        standard_action::AdvtAction,
    },
};

#[derive(Debug, Clone)]
pub struct AdvtExecuteSequenceDao {
    action: AdvtAction,
    condition: Option<Condition>,
    sequence: Option<i16>,
}

impl IsDao for AdvtExecuteSequenceDao {
    fn to_row(&self) -> Vec<msi::Value> {
        vec![
            str_val!(self.action),
            opt_str_val!(self.condition),
            opt_int_val!(self.sequence),
        ]
    }
}

impl ToOptionalIdentifier for AdvtExecuteSequenceDao {
    fn to_optional_identifier(&self) -> Option<Identifier> {
        None
    }
}

impl MsiBuilderListEntry for AdvtExecuteSequenceDao {
    fn conflicts(&self, other: &Self) -> bool {
        self.action == other.action
    }
}

impl From<AdvtAction> for AdvtExecuteSequenceDao {
    fn from(value: AdvtAction) -> Self {
        Self { action: value, condition: None, sequence: Some(value as i16) }
    }
}
