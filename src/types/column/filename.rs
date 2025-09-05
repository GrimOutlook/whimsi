use std::str::FromStr;

use anyhow::Context;
use anyhow::bail;
use anyhow::ensure;
use derive_more::Display;
use getset::Getters;
use itertools::Itertools;

use crate::constants::*;
use crate::types::helpers::invalid_char::InvalidChar;

#[derive(Clone, Debug, derive_more::Display, Default, Getters, PartialEq)]
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
            long: LongFilename::from_str(input).context(format!(
                "Failed parsing long filename from [{input}]"
            ))?,
        })
    }

    pub fn strict_parse(input: &str) -> anyhow::Result<Self> {
        Ok(Self {
            short: ShortFilename::from_str(input).context(format!(
                "Failed parsing short filename from [{input}]"
            ))?,
            long: LongFilename::from_str(input).context(format!(
                "Failed parsing long filename from [{input}]"
            ))?,
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

impl PartialOrd for Filename {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.to_string().partial_cmp(&other.to_string())
    }
}

impl Ord for Filename {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.to_string().cmp(&other.to_string())
    }
}

impl Eq for Filename {}

#[derive(Clone, Debug, derive_more::From, PartialEq, thiserror::Error)]
pub enum FilenameParsingError {
    #[error("Filename input string is blank")]
    EmptyString,

    #[error("Filename cannot end in period")]
    EndsWithPeriod,

    #[error(
        "Filename has invalid characters. Invalid characters: {:?}",
        LongFilename::INVALID_CHARS
    )]
    InvalidCharacters { characters: Vec<InvalidChar> },
}

#[derive(Clone, Debug, Display, Default, PartialEq)]
pub struct LongFilename {
    inner: String,
}
impl LongFilename {
    const INVALID_CHARS: &[char] =
        &['/', '\\', '?', '|', '>', '<', ':', '*', '"'];
}

impl FromStr for LongFilename {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s != "." {
            validate_long_filename(s)?;
        }

        Ok(LongFilename { inner: s.into() })
    }
}

/// Short filenames have the same restricted character setas long filenames but
/// with a few more characters added.
// TODO: Figure out if short filenames are allowed to be missing an extension.
// Documentation is unclear. Assuming yes.
#[derive(Clone, Debug, derive_more::Display, Default, PartialEq)]
pub struct ShortFilename {
    inner: String,
}
impl ShortFilename {
    const INVALID_CHARS: &[char] = &['+', ',', ';', '=', '[', ']'];

    pub fn trimmed(input: &str) -> anyhow::Result<Self> {
        let s = if input != "." {
            let file = std::path::Path::new(input);
            let Some(filename) = file.file_stem() else {
                bail!(ShortFilenameParsingError::NoFilename);
            };
            let filename = filename.to_str().context(format!(
                "Failed to convert filepath os_str {filename:?} to str"
            ))?;

            filename.get(0..SHORT_FILENAME_MAX_LEN).unwrap_or(filename)
        } else {
            input
        };
        Self::from_str(s)
    }
}

impl FromStr for ShortFilename {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s != "." {
            validate_long_filename(s)?;
            validate_short_filename(s)?;
        }

        Ok(ShortFilename { inner: s.into() })
    }
}

const TOO_LONG_ERR_MESSAGE: &str = "Short filename input is too long. Only 8 characters + period (.) + 3 letter extension allowed";

#[derive(Clone, Debug, derive_more::From, PartialEq, thiserror::Error)]
pub enum ShortFilenameParsingError {
    #[error(
        "Short filename has invalid characters. Invalid characters: {:?}",
        [ShortFilename::INVALID_CHARS, LongFilename::INVALID_CHARS].concat()
    )]
    InvalidCharacters { characters: Vec<InvalidChar> },

    #[error("{}", TOO_LONG_ERR_MESSAGE)]
    ExtensionTooLong,
    #[error("{}", TOO_LONG_ERR_MESSAGE)]
    FilenameTooLong,

    #[error("No filename found in short filename input")]
    NoFilename,
}

fn invalid_chars(invalid: &[char], haystack: &str) -> Vec<InvalidChar> {
    haystack
        .char_indices()
        .filter(|(_pos, ch)| invalid.contains(ch))
        .map(|(pos, ch)| InvalidChar::new(ch, pos))
        .collect_vec()
}

fn validate_long_filename(s: &str) -> anyhow::Result<()> {
    ensure!(!s.is_empty(), FilenameParsingError::EmptyString);
    ensure!(!s.ends_with("."), FilenameParsingError::EndsWithPeriod);
    let invalid_chars = invalid_chars(LongFilename::INVALID_CHARS, s);
    ensure!(
        invalid_chars.is_empty(),
        FilenameParsingError::InvalidCharacters { characters: invalid_chars }
    );
    Ok(())
}

fn validate_short_filename(s: &str) -> anyhow::Result<()> {
    let invalid_chars = invalid_chars(ShortFilename::INVALID_CHARS, s);
    ensure!(
        invalid_chars.is_empty(),
        ShortFilenameParsingError::InvalidCharacters {
            characters: invalid_chars,
        }
    );
    let file = std::path::Path::new(s);
    if let Some(stem) = file.file_stem() {
        ensure!(stem.len() <= 8, ShortFilenameParsingError::FilenameTooLong);
    } else {
        bail!(ShortFilenameParsingError::NoFilename);
    };

    if let Some(ext) = file.extension()
        && ext.len() > 3
    {
        bail!(ShortFilenameParsingError::ExtensionTooLong);
    };
    Ok(())
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use test_case::test_case;

    use crate::types::column::filename::LongFilename;
    use crate::types::column::filename::ShortFilename;
    use crate::types::column::filename::ShortFilenameParsingError;
    use crate::types::helpers::invalid_char::InvalidChar;

    const LONG_INVALID_PANIC_MSG: &str =
        "VALID long filename is evaluating as INVALID";
    const SHORT_INVALID_PANIC_MSG: &str =
        "VALID short filename is evaluating as INVALID";
    const LONG_VALID_PANIC_MSG: &str =
        "INVALID long filename is evaluating as VALID";
    const SHORT_VALID_PANIC_MSG: &str =
        "INVALID short filename is evaluating as VALID";

    use super::FilenameParsingError;
    #[test_case("long_filenae.ext"; "normal long")]
    fn valid_only_long(input: &str) {
        let expected = LongFilename { inner: input.into() };
        let actual =
            LongFilename::from_str(input).expect(LONG_INVALID_PANIC_MSG);
        assert_eq!(expected, actual)
    }

    #[test_case("filename.ext"; "normal")]
    #[test_case("filename"; "no extension")]
    #[test_case(".file.ext"; "starts with period")]
    fn valid_long_and_short(input: &str) {
        let expected_long = LongFilename { inner: input.into() };
        let actual_long =
            LongFilename::from_str(input).expect(LONG_INVALID_PANIC_MSG);
        let expected_short = ShortFilename { inner: input.into() };
        let actual_short =
            ShortFilename::from_str(input).expect(SHORT_INVALID_PANIC_MSG);
        assert_eq!(expected_long, actual_long, "long");
        assert_eq!(expected_short, actual_short, "short");
    }

    #[test_case("", FilenameParsingError::EmptyString ; "empty string")]
    #[test_case("filename.", FilenameParsingError::EndsWithPeriod ; "ends with period")]
    #[test_case("fi:le", FilenameParsingError::InvalidCharacters { characters: vec![InvalidChar::new(':', 2)] } ; "contains colon")]
    fn invalid_long_and_short(input: &str, expected: FilenameParsingError) {
        let long_actual = LongFilename::from_str(input);
        let short_actual = ShortFilename::from_str(input);
        assert_eq!(
            expected,
            short_actual.expect_err(SHORT_VALID_PANIC_MSG).downcast().unwrap()
        );
        assert_eq!(
            expected,
            long_actual.expect_err(LONG_VALID_PANIC_MSG).downcast().unwrap()
        );
    }

    #[test_case("f,ile",ShortFilenameParsingError::InvalidCharacters { characters: vec![InvalidChar::new(',', 1)] } ; "contains comma")]
    #[test_case("long_filename", ShortFilenameParsingError::FilenameTooLong; "long filename")]
    #[test_case("long.extension", ShortFilenameParsingError::ExtensionTooLong; "long extension")]
    fn invalid_short(input: &str, expected: ShortFilenameParsingError) {
        let actual =
            ShortFilename::from_str(input).expect_err(SHORT_VALID_PANIC_MSG);
        assert_eq!(
            expected,
            actual.downcast().unwrap_or_else(|_| panic!(
                "ShortFilenameError is incorrect type"
            ))
        );
    }
}
