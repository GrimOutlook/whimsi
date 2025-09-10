use getset::Getters;

use crate::opt_int_val;
use crate::opt_str_val;
use crate::str_val;
use crate::tables::builder_list_entry::MsiBuilderListEntry;
use crate::tables::dao::IsDao;
use crate::tables::generic_sequence::action_identifier::ActionIdentifier;
use crate::types::column::condition::Condition;
use crate::types::column::identifier::Identifier;
use crate::types::helpers::to_unique_msi_identifier::ToUniqueMsiIdentifier;

#[derive(Debug, Clone, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct GenericSequenceDao {
    action: ActionIdentifier,
    condition: Option<Condition>,
    sequence: Option<i16>,
}

impl IsDao for GenericSequenceDao {
    fn to_row(&self) -> Vec<msi::Value> {
        vec![
            str_val!(self.action),
            opt_str_val!(self.condition),
            opt_int_val!(self.sequence),
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
