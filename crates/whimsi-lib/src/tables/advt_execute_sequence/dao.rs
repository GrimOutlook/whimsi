use crate::constants::*;
use crate::constants::{self};
use crate::tables::builder_list_entry::MsiBuilderListEntry;
use crate::tables::dao::IsDao;
use crate::types::column::condition::Condition;
use crate::types::column::identifier::Identifier;
use crate::types::column::sequence::Sequence;
use crate::types::helpers::to_msi_value::ToMsiOptionalValue;
use crate::types::helpers::to_unique_msi_identifier::ToUniqueMsiIdentifier;
use crate::types::standard_action::AdvtAction;

#[derive(Debug, Clone)]
pub struct AdvtExecuteSequenceDao {
    action: AdvtAction,
    condition: Option<Condition>,
    sequence: Option<i16>,
}

impl IsDao for AdvtExecuteSequenceDao {
    fn to_row(&self) -> Vec<whimsi_msi::Value> {
        vec![
            self.action.to_string().into(),
            self.condition.to_optional_value(),
            self.sequence.to_optional_value(),
        ]
    }
}

impl ToUniqueMsiIdentifier for AdvtExecuteSequenceDao {
    fn to_unique_msi_identifier(&self) -> Option<Identifier> {
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
