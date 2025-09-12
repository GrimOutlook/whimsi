use bitmask_enum::bitmask;

#[bitmask(i16)]
pub enum ComponentAttributes {
    LocalOnly  = 0x0000,
    SourceOnly = 0x0001,
    Optional   = 0x0002,
    // TODO: The rest
}
