#[derive(Clone, Copy, Debug, PartialEq, whimsi_macros::IntToValue)]
#[repr(i32)]
pub enum LockPermission {
    /// Read, write, and execute access
    GENERIC_ALL = 0x10000000,
    /// Execute access
    GENERIC_EXECUTE = 0x20000000,
    /// Write access
    GENERIC_WRITE = 0x40000000,
}
