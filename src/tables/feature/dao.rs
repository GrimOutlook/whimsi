use getset::Getters;

use crate::constants::*;
use crate::int_val;
use crate::opt_str_val;
use crate::opt_val;
use crate::str_val;
use crate::tables::builder_list_entry::MsiBuilderListEntry;
use crate::tables::dao::IsDao;
use crate::tables::feature::identifier::FeatureIdentifier;
use crate::types::column::identifier::Identifier;
use crate::types::column::identifier::ToIdentifier;
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
