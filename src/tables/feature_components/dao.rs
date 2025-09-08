use getset::Getters;

use crate::tables::{
    component::table::ComponentIdentifier,
    feature::identifier::FeatureIdentifier,
};

#[derive(Debug, Clone, Getters, derive_more::Constructor)]
#[getset(get = "pub")]
pub struct FeatureComponentsDao {
    feature: FeatureIdentifier,
    component: ComponentIdentifier,
}
