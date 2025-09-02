use crate::types::helpers::filename::Filename;

use super::identifier::Identifier;

#[derive(Clone, Debug, derive_more::Display, derive_more::From, PartialEq)]
pub enum DefaultDir {
    Identifier(Identifier),
    Filename(Filename),
}
