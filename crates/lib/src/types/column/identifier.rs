use std::str::FromStr;

use ambassador::delegatable_trait;
use ambassador::delegatable_trait_remote;
use anyhow::Context;
use anyhow::bail;
use derive_more::Display;
use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;
use strum::IntoEnumIterator;
use thiserror::Error;

use super::ColumnValue;
use crate::types::helpers::invalid_char::InvalidChar;
use crate::types::helpers::to_msi_value::IntoMsiValue;
use crate::types::properties::SystemFolderProperty;

static INVALID_FIRST_CHARACTER: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[^A-Za-z_]").unwrap());
static INVALID_CHARACTER: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"[^A-Za-z0-9_\.]").unwrap());

/// May only contain ASCII characters of the set [A-Za-z0-9_\.]
/// Must start with either a letter or underscore.
///
/// Reference: https://learn.microsoft.com/en-us/windows/win32/msi/identifier
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    Hash,
    derive_more::Display,
    whimsi_macros::StrToValue,
)]
pub struct Identifier(String);

impl Identifier {
    pub fn as_system_folder(&self) -> Option<SystemFolderProperty> {
        SystemFolderProperty::iter().find(|f| f == self)
    }
}

#[delegatable_trait]
pub trait IntoIdentifier {
    fn into(&self) -> Identifier;
}

impl IntoIdentifier for Identifier {
    fn into(&self) -> Identifier {
        self.clone()
    }
}

impl FromStr for Identifier {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(hit) = INVALID_FIRST_CHARACTER.find(s) {
            bail!(IdentifierConversionError::InvalidFirstCharacter {
                first_character: InvalidChar::new(
                    hit.as_str().chars().next().unwrap(),
                    0
                ),
            });
        }

        if INVALID_CHARACTER.is_match(s) {
            let characters = INVALID_CHARACTER
                .find_iter(s)
                .enumerate()
                .map(|(index, hit)| {
                    InvalidChar::new(
                        hit.as_str().chars().next().unwrap(),
                        index,
                    )
                })
                .collect_vec();
            bail!(IdentifierConversionError::InvalidCharacters { characters });
        }

        Ok(Identifier(s.to_string()))
    }
}

impl From<&SystemFolderProperty> for Identifier {
    fn from(value: &SystemFolderProperty) -> Self {
        value
            .to_string()
            .parse()
            .context(format!(
                "Failed to parse system folder {value:?} to identifier"
            ))
            .unwrap()
    }
}

impl From<SystemFolderProperty> for Identifier {
    fn from(value: SystemFolderProperty) -> Self {
        (&value).into()
    }
}

#[derive(Debug, Error, PartialEq)]
pub enum IdentifierConversionError {
    #[error("Identifier has invalid first character: [{first_character}]")]
    InvalidFirstCharacter { first_character: InvalidChar },
    #[error("Identifier contains invalid characters")]
    InvalidCharacters { characters: Vec<InvalidChar> },
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use test_case::test_case;

    use super::Identifier;
    use crate::types::column::identifier::IdentifierConversionError;
    use crate::types::helpers::invalid_char::InvalidChar;
    #[test_case("Test8."; "starts with letter")]
    #[test_case("_Test8."; "starts with underscore")]
    fn valid_identifier(input: &str) {
        let expected = Identifier(input.to_owned());
        let actual = Identifier::from_str(input)
            .expect("Valid identifier returning as invalid");
        assert_eq!(expected, actual);
    }

    #[test_case(".Test"; "starts with period")]
    #[test_case("8Test"; "starts with number")]
    fn invalid_first_character(input: &str) {
        let actual = Identifier::from_str(input)
            .expect_err("Invalid identifier is returning as valid")
            .downcast()
            .unwrap();
        let expected = IdentifierConversionError::InvalidFirstCharacter {
            first_character: InvalidChar::new(
                input.chars().next().unwrap(),
                0,
            ),
        };
        assert_eq!(expected, actual);
    }
}
