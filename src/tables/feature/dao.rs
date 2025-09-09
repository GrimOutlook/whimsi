use getset::Getters;

use crate::{
    constants::*,
    int_val, opt_str_val, opt_val, str_val,
    tables::{
        builder_list_entry::MsiBuilderListEntry, dao::IsDao,
        feature::identifier::FeatureIdentifier,
    },
    types::column::identifier::{
        Identifier, ToIdentifier, ToOptionalIdentifier,
    },
};

#[derive(Clone, Debug, PartialEq, Getters)]
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
    pub fn new(
        feature_identifier: FeatureIdentifier,
        title: impl ToString,
    ) -> FeatureDao {
        Self {
            feature: feature_identifier,
            title: Some("Default Feature".to_owned()),
            ..Default::default()
        }
    }
}

impl IsDao for FeatureDao {
    fn to_row(&self) -> Vec<msi::Value> {
        vec![
            str_val!(self.feature),
            opt_str_val!(self.feature_parent),
            opt_str_val!(self.title),
            opt_str_val!(self.description),
            opt_val!(self.display),
            int_val!(self.level),
            opt_str_val!(self.directory),
            int_val!(self.attributes),
        ]
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

impl Default for FeatureDao {
    fn default() -> Self {
        Self {
            feature: DEFAULT_FEATURE_IDENTIFIER.parse().unwrap(),
            title: Some(DEFAULT_FEATURE_TITLE.to_string()),
            display: Some(DEFAULT_FEATURE_DISPLAY),
            level: DEFAULT_FEATURE_LEVEL,
            feature_parent: Default::default(),
            description: Default::default(),
            directory: Default::default(),
            attributes: Default::default(),
        }
    }
}
