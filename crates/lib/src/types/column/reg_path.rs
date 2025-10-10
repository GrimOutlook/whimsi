use std::str::FromStr;

use anyhow::ensure;

use crate::types::column::formatted::Formatted;

/// Formatted string that cannot start or end with backslashes.
#[derive(
    Debug, Clone, PartialEq, derive_more::Display, whimsi_macros::StrToValue,
)]
pub struct RegPath(Formatted);

impl FromStr for RegPath {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        ensure!(
            !s.starts_with(r"\"),
            "RegPath starts with backslash. Not allowed."
        );
        ensure!(
            !s.ends_with(r"\"),
            "RegPath ends with backslash. Not allowed."
        );
        Ok(RegPath(s.to_string().into()))
    }
}
