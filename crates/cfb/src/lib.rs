use std::{io::Read, path::Path};
struct CfbHeader;
struct FatSector;
struct DifatSector;
struct MiniFatSector;
struct DirectorySector;

struct Cfb {
    header: CfbHeader,
    fat_sectors: Vec<FatSector>,
    difat_sectors: Vec<DifatSector>,
    minifat_sectors: Vec<MiniFatSector>,
    directory_sectors: Vec<DirectorySector>,
}

const MAGIC_NUMBERS: &[u8; 8] =
    &[0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1];
const RESERVED_FIELD_LENGTH: usize = 16;

fn open(path: impl AsRef<Path>) -> anyhow::Result<Cfb> {
    let mut file = std::fs::File::open(path)?;

    // Verify that the magic number of the file matches the magic number of a standard CFB file.
    let mut magic_numbers = [0; MAGIC_NUMBERS.len()];
    file.read_exact(&mut magic_numbers)?;
    if magic_numbers != *MAGIC_NUMBERS {
        panic!("Magic Numbers don't match")
    }

    // Read in the reserved field bytes.
    file.read_exact(&mut [0u8; RESERVED_FIELD_LENGTH])?;

    todo!()
}
