use crate::{
    tables::{builder_list_entry::MsiBuilderListEntry, dao::IsDao},
    types::{
        column::{
            condition::{self, Condition},
            formatted::Formatted,
            identifier::Identifier,
        },
        helpers::to_unique_msi_identifier::ToUniqueMsiIdentifier,
    },
};

#[derive(Debug, Clone, PartialEq)]
pub struct LaunchConditionDao {
    condition: Condition,
    description: Formatted,
}

impl MsiBuilderListEntry for LaunchConditionDao {
    fn conflicts(&self, other: &Self) -> bool {
        self == other
    }
}

impl ToUniqueMsiIdentifier for LaunchConditionDao {
    fn to_unique_msi_identifier(&self) -> Option<Identifier> {
        None
    }
}

impl IsDao for LaunchConditionDao {
    fn to_row(&self) -> Vec<msi::Value> {
        vec![self.condition.clone().into(), self.description.clone().into()]
    }
}
