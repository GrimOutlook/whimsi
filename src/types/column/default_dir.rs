use super::identifier::Identifier;
use crate::types::helpers::filename::Filename;

#[derive(Clone, Debug, derive_more::Display, derive_more::From, PartialEq)]
pub enum DefaultDir {
    Identifier(Identifier),
    Filename(Filename),
}
