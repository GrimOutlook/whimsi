use std::str::FromStr;

use anyhow::Context;
use anyhow::ensure;

#[derive(Clone, Debug, PartialEq, derive_more::Display)]
pub struct Version(String);
impl FromStr for Version {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Yes I know it's not actually an octet since there aren't 8 but people
        // still call each part of an IP address an octet :p
        let octets = s.split(".");

        /// 1-4 octets are allowed.
        ensure!(
            octets.clone().count() <= 4,
            VersionError::TooManyOctets(s.to_owned())
        );

        /// Verify that each octet can be represented as a u16 since version
        /// octets cannot be a value greater than 65535.
        for (index, octet) in octets.enumerate() {
            octet.parse::<u16>().context(format!(
                "Version octet [{}] in position [{}] could not be parsed as a number (u16)",
                octet, index
            ))?;
        }

        Ok(Version(s.to_owned()))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum VersionError {
    #[error("Input string [{0}] has too many octets")]
    TooManyOctets(String),
    #[error(
        "Octet [{0}] at index [{1}] is too large to be represented by an MSI version. Maximum value is 65535."
    )]
    OctetTooLarge(String, u8),
    #[error(
        "Octet [{0}] at index [{1}] is is not a number. Only numbers allowed in MSI version."
    )]
    OctetNotANumber(String, u8),
}
