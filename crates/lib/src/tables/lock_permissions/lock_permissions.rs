use msi::Value;

#[derive(Debug, Clone)]
pub enum LockPermissions {
    /// Read, write, and execute access
    GENERIC_ALL,
    /// Execute access
    GENERIC_EXECUTE,
    /// Write access
    GENERIC_WRITE,
}

impl From<LockPermissions> for Value {
    fn from(value: LockPermissions) -> Value {
        (value as i16).into()
    }
}
