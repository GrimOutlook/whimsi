use super::{filename::MsiFilename, identifier::Identifier};

#[derive(Clone, Debug, PartialEq)]
pub enum DefaultDir {
    Identifier(Identifier),
    Filename(MsiFilename),
}
