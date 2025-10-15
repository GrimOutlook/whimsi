use whimsi_msi::Value;

#[derive(Debug, Clone, serde::Deserialize)]
#[repr(i32)]
pub enum LockPermissions {
    /// Read, write, and execute access
    ALL     = 0x10000000,
    /// Execute access
    EXECUTE = 0x20000000,
    /// Write access
    WRITE   = 0x40000000,
}

impl From<LockPermissions> for Value {
    fn from(value: LockPermissions) -> Value {
        (value as i16).into()
    }
}
