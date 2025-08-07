use once_cell::sync::Lazy;
use std::str::FromStr;

use regex::Regex;

static VALID_IDENTIFIER: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[A-Za-z_\.]*$").unwrap());
static INVALID_FIRST_CHARACTER: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[^[A-Z][a-z]_]").unwrap());
static INVALID_CHARACTER: Lazy<Regex> = Lazy::new(|| Regex::new(r"[^A-Za-z_\.]*").unwrap());

/// May only contain ASCII characters of the set [A-Za-z0-9_\.]
/// Must start with either a letter or underscore.
pub struct Identifier {
    inner: String,
}

impl FromStr for Identifier {
    type Err = IdentifierConversionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Return early if the identifier is found to be valid.
        if VALID_IDENTIFIER.is_match(s) {
            return Ok(Identifier {
                inner: s.to_string(),
            });
        }

        // Determine error for so it can be reported to the user.

        if let Some(hit) = INVALID_FIRST_CHARACTER.find(s) {
            return Err(IdentifierConversionError::InvalidFirstCharacter {
                first_character: hit.as_str().into(),
                input_string: s.into(),
            });
        }
        todo!()
    }
}

// TODO: Optimization: See if these `String`s can be changed to `&str`. Tried to do it initially
// but ran into E0207 since neither `FromStr` nor `Identifier` accept a lifetime (nor should they)
pub enum IdentifierConversionError {
    InvalidFirstCharacter {
        first_character: String,
        input_string: String,
    },
    InvalidCharacter {
        invalid_character: String,
        input_string: String,
        position: usize,
    },
}

#[cfg(test)]
mod test {
    use test_case::test_case;
    #[test_case];
    fn from_str() {
        todo!();
    }
}
