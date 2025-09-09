use std::str::FromStr;

use anyhow::ensure;

/// Guaruntees that the text is not an empty string.
#[derive(Clone, Debug, Default, derive_more::Display)]
pub(crate) struct PropertyText(String);

impl FromStr for PropertyText {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        ensure!(
            s != "" && !s.is_empty(),
            "Property value is an emptry string. Not allowed."
        );
        Ok(PropertyText(s.to_string()))
    }
}
