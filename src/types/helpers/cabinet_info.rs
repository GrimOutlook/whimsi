use std::path::PathBuf;

use getset::Getters;

use crate::types::column::identifier::Identifier;

#[derive(Debug, Clone, Default, Getters)]
#[getset(get = "pub")]
pub struct CabinetInfo {
    id: Identifier,
    files: Vec<CabinetContainedFile>,
}

impl CabinetInfo {
    pub fn new(id: Identifier) -> Self {
        Self { id, files: Vec::new() }
    }

    pub fn add_file(&mut self, id: Identifier, path: PathBuf) {
        self.files.push(CabinetContainedFile { id, path });
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

#[derive(Debug, Clone, Getters)]
#[getset(get = "pub")]
pub struct CabinetContainedFile {
    id: Identifier,
    path: PathBuf,
    // size: i32,
}
