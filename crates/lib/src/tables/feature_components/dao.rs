use getset::Getters;

use crate::tables::builder_list_entry::MsiBuilderListEntry;
use crate::tables::component::table::ComponentIdentifier;
use crate::tables::dao::IsDao;
use crate::tables::feature::identifier::FeatureIdentifier;
use crate::types::column::identifier::Identifier;
use crate::types::helpers::to_unique_msi_identifier::ToUniqueMsiIdentifier;

#[derive(Debug, Clone, Getters, PartialEq, derive_more::Constructor)]
#[getset(get = "pub")]
pub struct FeatureComponentsDao {
    feature: FeatureIdentifier,
    component: ComponentIdentifier,
}

impl IsDao for FeatureComponentsDao {
    fn to_row(&self) -> Vec<whimsi_msi::Value> {
        vec![self.feature.clone().into(), self.component.clone().into()]
    }
}

impl MsiBuilderListEntry for FeatureComponentsDao {
    fn conflicts(&self, other: &Self) -> bool {
        self == other
    }
}

impl ToUniqueMsiIdentifier for FeatureComponentsDao {
    fn to_unique_msi_identifier(&self) -> Option<Identifier> {
        None
    }
}
