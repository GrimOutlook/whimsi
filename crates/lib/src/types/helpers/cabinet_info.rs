use std::path::PathBuf;

use getset::Getters;

use crate::{
    tables::FileIdentifier,
    types::{
        column::identifier::{Identifier, ToIdentifier},
        helpers::{
            cabinets::CabinetIdentifier, primary_identifier::PrimaryIdentifier,
        },
    },
};

#[derive(Debug, Clone, Default, Getters, PartialEq)]
#[getset(get = "pub")]
pub struct CabinetInfo {
    id: CabinetIdentifier,
    files: Vec<CabinetContainedFile>,
}

impl CabinetInfo {
    pub fn new(id: CabinetIdentifier) -> Self {
        Self { id, files: Vec::new() }
    }

    pub fn add_file(&mut self, id: FileIdentifier, path: PathBuf) {
        self.files.push(CabinetContainedFile { id, path });
    }

    fn conflicts(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl PrimaryIdentifier for CabinetInfo {
    fn primary_identifier(&self) -> Option<Identifier> {
        Some(self.id.to_identifier())
    }
}

// NOTE: I initially added the size here instead of just tracking file
// identifiers because in my head there was a max size for a cabinet file. I
// can't seem to find any documentation on that so I'm leaving it as a vestige.
// May later attempt to figure out what kind of partition is being used and not
// create cabinet files larger than can be represented.
// TODO: Determine if there
// is any need to track file sizes in a cabinet file or the size of the
// cabinet file itself.

#[derive(Debug, Clone, Getters, PartialEq)]
#[getset(get = "pub")]
pub struct CabinetContainedFile {
    id: FileIdentifier,
    path: PathBuf,
    // size: i32,
}
