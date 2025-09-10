use getset::Getters;

use crate::constants::*;
use crate::tables::builder_list_entry::MsiBuilderListEntry;
use crate::tables::dao::IsDao;
use crate::tables::feature::identifier::FeatureIdentifier;
use crate::types::column::identifier::Identifier;
use crate::types::column::identifier::ToIdentifier;
use crate::types::helpers::to_msi_value::ToMsiOptionalValue;
use crate::types::helpers::to_unique_msi_identifier::ToUniqueMsiIdentifier;

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
            self.feature.clone().into(),
            self.feature_parent.to_optional_value(),
            self.title.to_optional_value(),
            self.description.to_optional_value(),
            self.display.to_optional_value(),
            self.level.into(),
            self.directory.to_optional_value(),
            self.attributes.into(),
        ]
    }
}
impl MsiBuilderListEntry for FeatureDao {
    fn conflicts(&self, other: &Self) -> bool {
        self.feature == other.feature
    }
}

impl ToUniqueMsiIdentifier for FeatureDao {
    fn to_unique_msi_identifier(&self) -> Option<Identifier> {
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
