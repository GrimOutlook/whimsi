use derive_more::From;

use crate::types::helpers::filename::Filename;

use super::identifier::Identifier;

#[derive(Clone, Debug, From, PartialEq)]
pub enum DefaultDir {
    Identifier(Identifier),
    Filename(Filename),
}
