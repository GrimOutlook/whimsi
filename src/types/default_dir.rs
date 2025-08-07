use super::{filename::Filename, identifier::Identifier};

pub enum DefaultDir {
    Identifier(Identifier),
    Filename(Filename),
}
