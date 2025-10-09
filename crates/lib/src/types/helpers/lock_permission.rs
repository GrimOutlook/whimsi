#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(i32)]
pub enum LockPermission {
    /// Read, write, and execute access
    GENERIC_ALL = 0x10000000,
    /// Execute access
    GENERIC_EXECUTE = 0x20000000,
    /// Write access
    GENERIC_WRITE = 0x40000000,
}

impl msi::ToValue for LockPermission {
    fn to_value(&self) -> msi::Value {
        msi::Value::Int(*self as i32)
    }
}
