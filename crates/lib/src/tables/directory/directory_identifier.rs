use std::str::FromStr;

use crate::types::column::identifier::Identifier;
use crate::types::column::identifier::ToIdentifier;
use crate::types::column::identifier::ambassador_impl_ToIdentifier;
use crate::types::helpers::to_unique_msi_identifier::ToUniqueMsiIdentifier;
use crate::types::helpers::to_unique_msi_identifier::ambassador_impl_ToUniqueMsiIdentifier;
use crate::types::properties::system_folder::SystemFolder;

#[derive(
    Debug,
    Clone,
    PartialEq,
    ambassador::Delegate,
    derive_more::Display,
    derive_more::From,
    whimsi_macros::IntoStrMsiValue,
)]
#[delegate(ToIdentifier)]
pub enum DirectoryIdentifier {
    SystemFolder(SystemFolder),
    Identifier(Identifier),
}

impl FromStr for DirectoryIdentifier {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        Ok(DirectoryIdentifier::Identifier(Identifier::from_str(s)?))
    }
}
