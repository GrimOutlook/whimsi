use getset::Getters;

use crate::{tables::dao::IsDao, types::column::identifier::Identifier};

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

impl IsDao for FeatureDao {
    fn to_row(&self) -> Vec<msi::Value> {
        vec![]
    }

    fn conflicts(&self, other: &Self) -> bool {
        self.feature == other.feature
    }
}
