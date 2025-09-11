use getset::Getters;

use crate::tables::builder_list_entry::MsiBuilderListEntry;
use crate::tables::dao::IsDao;
use crate::tables::generic_sequence::action_identifier::ActionIdentifier;
use crate::types::column::condition::Condition;
use crate::types::column::identifier::Identifier;
use crate::types::helpers::to_msi_value::ToMsiOptionalValue;
use crate::types::helpers::to_unique_msi_identifier::ToUniqueMsiIdentifier;
use crate::types::standard_action::StandardAction;

#[derive(Debug, Clone, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct GenericSequenceDao {
    action: ActionIdentifier,
    condition: Option<Condition>,
    sequence: Option<i16>,
}

impl IsDao for GenericSequenceDao {
    fn to_row(&self) -> Vec<whimsi_msi::Value> {
        vec![
            self.action.clone().into(),
            self.condition.to_optional_value(),
            self.sequence.to_optional_value(),
        ]
    }
}

impl MsiBuilderListEntry for GenericSequenceDao {
    fn conflicts(&self, other: &Self) -> bool {
        self.action.to_string() == other.action().to_string()
    }
}

impl ToUniqueMsiIdentifier for GenericSequenceDao {
    fn to_unique_msi_identifier(&self) -> Option<Identifier> {
        None
    }
}

impl From<StandardAction> for GenericSequenceDao {
    fn from(value: StandardAction) -> Self {
        Self {
            action: value.into(),
            condition: None,
            sequence: Some(value as i16),
        }
    }
}
