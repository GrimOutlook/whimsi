use std::str::FromStr;

use anyhow::Context;
use derive_more::Display;
use getset::Getters;

use crate::types::column::filename::{LongFilename, ShortFilename};

#[derive(Clone, Debug, Display, Getters, PartialEq)]
#[display("{}", long)]
#[get = "pub"]
pub struct Filename {
    short: ShortFilename,
    long: LongFilename,
}

impl Filename {
    pub fn parse(input: &str) -> anyhow::Result<Self> {
        Ok(Self {
            short: ShortFilename::trimmed(input).context(format!(
                "Failed parsing short filename with trim from [{input}]"
            ))?,
            long: LongFilename::from_str(input)
                .context(format!("Failed parsing long filename from [{input}]"))?,
        })
    }

    pub fn strict_parse(input: &str) -> anyhow::Result<Self> {
        Ok(Self {
            short: ShortFilename::from_str(input)
                .context(format!("Failed parsing short filename from [{input}]"))?,
            long: LongFilename::from_str(input)
                .context(format!("Failed parsing long filename from [{input}]"))?,
        })
    }
}

impl FromStr for Filename {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Filename::parse(s)
    }
}

impl From<Filename> for String {
    fn from(value: Filename) -> Self {
        format!("{}|{}", value.short, value.long)
    }
}
