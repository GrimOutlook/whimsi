use bitmask_enum::bitmask;

#[bitmask(i16)]
pub enum FileAttributes {
    ReadOnly = 0x0001,
    Hidden = 0x0002,
    System = 0x0004,
    Vital = 0x0200,
    Checksum = 0x0400,
    PatchAdded = 0x1000,
    NonCompressed = 0x2000,
    Compressed = 0x4000,
}
