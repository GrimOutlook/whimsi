use bitmask_enum::bitmask;

#[derive(Default)]
#[bitmask]
pub enum ComponentAttributes {
    #[default]
    LocalOnly = 0x0000,
    SourceOnly = 0x0001,
    Optional = 0x0002,
    // TODO: The rest
}
