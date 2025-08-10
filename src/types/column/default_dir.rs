use derive_more::From;

use super::{filename::MsiFilename, identifier::Identifier};

#[derive(Clone, Debug, From, PartialEq)]
pub enum DefaultDir {
    Identifier(Identifier),
    Filename(MsiFilename),
}
