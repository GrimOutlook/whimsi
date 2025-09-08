use std::str::FromStr;

use anyhow::ensure;

use crate::{
    constants::*,
    define_specific_identifier,
    types::column::identifier::{Identifier, ToIdentifier},
};

define_specific_identifier!(feature);

impl FromStr for FeatureIdentifier {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        ensure!(
            s.len() <= FEATURE_IDENTIFIER_MAX_LEN,
            "Feature Identifier is too long"
        );
        Ok(Self(Identifier::from_str(s)?))
    }
}
