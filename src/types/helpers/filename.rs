use std::str::FromStr;

use anyhow::Context;
use derive_more::Display;

use crate::types::column::filename::{LongFilename, ShortFilename};

#[derive(Clone, Debug, Display, PartialEq)]
#[display("{}", long)]
pub struct Filename {
    short: ShortFilename,
    long: LongFilename,
}

impl Filename {
    pub fn parse_with_trim(input: &str) -> anyhow::Result<Self> {
        Ok(Self {
            short: ShortFilename::trimmed(input).context(format!(
                "Failed parsing short filename with trim from [{input}]"
            ))?,
            long: LongFilename::from_str(input)
                .context(format!("Failed parsing long filename from [{input}]"))?,
        })
    }

    pub fn parse(input: &str) -> anyhow::Result<Self> {
        Ok(Self {
            short: ShortFilename::from_str(input)
                .context(format!("Failed parsing short filename from [{input}]"))?,
            long: LongFilename::from_str(input)
                .context(format!("Failed parsing long filename from [{input}]"))?,
        })
    }
}
