use std::str::FromStr;

use derive_more::Display;

use crate::{
    constants::*,
    types::column::filename::{LongFilename, ShortFilename},
};

#[derive(Clone, Debug, Display, PartialEq)]
#[display("{}", long)]
pub struct Filename {
    short: ShortFilename,
    long: LongFilename,
}

impl Filename {
    pub fn parse_with_trim(input: &str) -> anyhow::Result<Self> {
        Ok(Self {
            short: ShortFilename::trimmed(input)?,
            long: LongFilename::from_str(input)?,
        })
    }

    pub fn parse(input: &str) -> anyhow::Result<Self> {
        Ok(Self {
            short: ShortFilename::from_str(input)?,
            long: LongFilename::from_str(input)?,
        })
    }
}
