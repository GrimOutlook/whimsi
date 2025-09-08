use getset::Getters;

use crate::{
    tables::{
        builder_list_entry::MsiBuilderListEntry, dao::IsDao,
        feature::identifier::FeatureIdentifier,
    },
    types::column::identifier::{
        Identifier, ToIdentifier, ToOptionalIdentifier,
    },
};

#[derive(Clone, Debug, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct FeatureDao {
    feature: FeatureIdentifier,
    feature_parent: Option<FeatureIdentifier>,
    title: Option<String>,
    description: Option<String>,
    display: Option<i16>,
    level: i16,
    directory: Option<Identifier>,
    attributes: i16,
}

impl FeatureDao {
    pub fn new(feature_identifier: FeatureIdentifier) -> FeatureDao {
        Self {
            feature: feature_identifier,
            title: Some("Default Feature".to_owned()),
            display: Some(2),
            level: 1,
            ..Default::default()
        }
    }
}

impl IsDao for FeatureDao {
    fn to_row(&self) -> Vec<msi::Value> {
        vec![]
    }
}
impl MsiBuilderListEntry for FeatureDao {
    fn conflicts(&self, other: &Self) -> bool {
        self.feature == other.feature
    }
}

impl ToOptionalIdentifier for FeatureDao {
    fn to_optional_identifier(&self) -> Option<Identifier> {
        Some(self.feature.to_identifier())
    }
}
