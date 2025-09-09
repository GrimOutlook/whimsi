use getset::Getters;

use crate::{
    str_val,
    tables::{
        builder_list_entry::MsiBuilderListEntry,
        component::table::ComponentIdentifier, dao::IsDao,
        feature::identifier::FeatureIdentifier,
    },
    types::column::identifier::{Identifier, ToOptionalIdentifier},
};

#[derive(Debug, Clone, Getters, PartialEq, derive_more::Constructor)]
#[getset(get = "pub")]
pub struct FeatureComponentsDao {
    feature: FeatureIdentifier,
    component: ComponentIdentifier,
}

impl IsDao for FeatureComponentsDao {
    fn to_row(&self) -> Vec<msi::Value> {
        vec![str_val!(self.feature), str_val!(self.component)]
    }
}

impl MsiBuilderListEntry for FeatureComponentsDao {
    fn conflicts(&self, other: &Self) -> bool {
        self == other
    }
}

impl ToOptionalIdentifier for FeatureComponentsDao {
    fn to_optional_identifier(&self) -> Option<Identifier> {
        None
    }
}
