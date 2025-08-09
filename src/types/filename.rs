use std::str::FromStr;

use derive_more::{Display, From};
use itertools::Itertools;

use super::helpers::invalid_char::InvalidChar;

/// Name of a file *or* folder.
///
/// Reference: https://learn.microsoft.com/en-us/windows/win32/msi/filename
// TODO: Figure out if filenames are allowed to end in a period. Assuming no.
#[derive(Clone, Debug, Display, From, PartialEq)]
pub enum Filename {
    Long(LongFilename),
    Short(ShortFilename),
}

#[derive(Clone, Debug, Display, From, PartialEq)]
pub enum FilenameParsingError {
    ShortFilenameParsingError(ShortFilenameParsingError),

    #[display("Filename input string is blank")]
    EmptyString,

    #[display("Filename cannot end in period")]
    EndsWithPeriod,

    #[display(
        "Filename has invalid characters. Invalid characters: {:?}",
        LongFilename::INVALID_CHARS
    )]
    InvalidCharacters {
        characters: Vec<InvalidChar>,
    },
}

#[derive(Clone, Debug, Display, PartialEq)]
pub struct LongFilename {
    inner: String,
}
impl LongFilename {
    const INVALID_CHARS: &[char] = &['/', '\\', '?', '|', '>', '<', ':', '*', '"'];
}

impl FromStr for LongFilename {
    type Err = FilenameParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        validate_long_filename(s)?;

        Ok(LongFilename { inner: s.into() })
    }
}

/// Short filenames have the same restricted character setas long filenames but with a few more
/// characters added.
// TODO: Figure out if short filenames are allowed to be missing an extension. Documentation is
// unclear. Assuming yes.
#[derive(Clone, Debug, Display, PartialEq)]
pub struct ShortFilename {
    inner: String,
}
impl ShortFilename {
    const INVALID_CHARS: &[char] = &['+', ',', ';', '=', '[', ']'];
}
impl FromStr for ShortFilename {
    type Err = FilenameParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        validate_long_filename(s)?;
        validate_short_filename(s)?;

        Ok(ShortFilename { inner: s.into() })
    }
}

const TOO_LONG_ERR_MESSAGE: &str =
    "Short filename input is too long. Only 8 characters + period (.) + 3 letter extension allowed";

#[derive(Clone, Debug, Display, PartialEq)]
pub enum ShortFilenameParsingError {
    #[display(
        "Short filename has invalid characters. Invalid characters: {:?}",
        ShortFilename::INVALID_CHARS
    )]
    InvalidCharacters { characters: Vec<InvalidChar> },

    #[display("{}", TOO_LONG_ERR_MESSAGE)]
    ExtensionTooLong,
    #[display("{}", TOO_LONG_ERR_MESSAGE)]
    FilenameTooLong,

    #[display("No filename found in short filename input")]
    NoFilename,
}

fn invalid_chars(invalid: &[char], haystack: &str) -> Vec<InvalidChar> {
    haystack
        .char_indices()
        .filter(|(_pos, ch)| invalid.contains(ch))
        .map(|(pos, ch)| InvalidChar::new(ch, pos))
        .collect_vec()
}

fn validate_long_filename(s: &str) -> Result<(), FilenameParsingError> {
    if s.is_empty() {
        return Err(FilenameParsingError::EmptyString);
    }
    if s.ends_with(".") {
        return Err(FilenameParsingError::EndsWithPeriod);
    }
    let invalid_chars = invalid_chars(LongFilename::INVALID_CHARS, s);
    if !invalid_chars.is_empty() {
        return Err(FilenameParsingError::InvalidCharacters {
            characters: invalid_chars,
        });
    }
    Ok(())
}

fn validate_short_filename(s: &str) -> Result<(), FilenameParsingError> {
    let invalid_chars = invalid_chars(ShortFilename::INVALID_CHARS, s);
    if !invalid_chars.is_empty() {
        return Err(FilenameParsingError::ShortFilenameParsingError(
            ShortFilenameParsingError::InvalidCharacters {
                characters: invalid_chars,
            },
        ));
    }
    let file = std::path::Path::new(s);
    if let Some(stem) = file.file_stem() {
        if stem.len() > 8 {
            return Err(ShortFilenameParsingError::FilenameTooLong.into());
        }
    } else {
        return Err(ShortFilenameParsingError::NoFilename.into());
    };

    if let Some(ext) = file.extension()
        && ext.len() > 3
    {
        return Err(ShortFilenameParsingError::ExtensionTooLong.into());
    };
    Ok(())
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use test_case::test_case;

    use crate::types::{
        filename::{LongFilename, ShortFilename, ShortFilenameParsingError},
        helpers::invalid_char::InvalidChar,
    };

    const LONG_INVALID_PANIC_MSG: &str = "VALID long filename is evaluating as INVALID";
    const SHORT_INVALID_PANIC_MSG: &str = "VALID short filename is evaluating as INVALID";
    const LONG_VALID_PANIC_MSG: &str = "INVALID long filename is evaluating as VALID";
    const SHORT_VALID_PANIC_MSG: &str = "INVALID short filename is evaluating as VALID";

    use super::FilenameParsingError;
    #[test_case("long_filenae.ext"; "normal long")]
    fn valid_only_long(input: &str) {
        let expected = LongFilename {
            inner: input.into(),
        };
        let actual = LongFilename::from_str(input).expect(LONG_INVALID_PANIC_MSG);
        assert_eq!(expected, actual)
    }

    #[test_case("filename.ext"; "normal")]
    #[test_case("filename"; "no extension")]
    #[test_case(".file.ext"; "starts with period")]
    fn valid_long_and_short(input: &str) {
        let expected_long = LongFilename {
            inner: input.into(),
        };
        let actual_long = LongFilename::from_str(input).expect(LONG_INVALID_PANIC_MSG);
        let expected_short = ShortFilename {
            inner: input.into(),
        };
        let actual_short = ShortFilename::from_str(input).expect(SHORT_INVALID_PANIC_MSG);
        assert_eq!(expected_long, actual_long, "long");
        assert_eq!(expected_short, actual_short, "short");
    }

    #[test_case("", FilenameParsingError::EmptyString ; "empty string")]
    #[test_case("filename.", FilenameParsingError::EndsWithPeriod ; "ends with period")]
    #[test_case("fi:le", FilenameParsingError::InvalidCharacters { characters: vec![InvalidChar::new(':', 2)] } ; "contains colon")]
    fn invalid_long_and_short(input: &str, expected: FilenameParsingError) {
        let long_actual = LongFilename::from_str(input);
        let short_actual = ShortFilename::from_str(input);
        assert_eq!(expected, short_actual.expect_err(SHORT_VALID_PANIC_MSG));
        assert_eq!(expected, long_actual.expect_err(LONG_VALID_PANIC_MSG));
    }

    #[test_case("f,ile", ShortFilenameParsingError::InvalidCharacters { characters: vec![InvalidChar::new(',', 1)] }.into() ; "contains comma")]
    #[test_case("long_filename", ShortFilenameParsingError::FilenameTooLong.into(); "long filename")]
    #[test_case("long.extension", ShortFilenameParsingError::ExtensionTooLong.into(); "long extension")]
    fn invalid_short(input: &str, expected: FilenameParsingError) {
        let actual = ShortFilename::from_str(input).expect_err(SHORT_VALID_PANIC_MSG);
        assert_eq!(expected, actual);
    }
}
