use std::{fmt::Display, str::FromStr};

use crate::types::{default_dir::DefaultDir, identifier::Identifier};

pub struct Directories(Vec<Directory>);
impl Directories {
    pub fn add(&mut self, directory: Directory) {
        self.0.push(directory)
    }
}

pub struct Directory {
    /// Name of this directory
    directory: Identifier,
    /// Directory that contains this directory
    parent: Identifier,
    /// The directory's name (localizable) under the parent directory
    default: DefaultDir,
}

impl FromStr for Directory {
    type Err = DirectoryConversionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

pub enum DirectoryConversionError {
    InvalidFirstCharacter(),
}
