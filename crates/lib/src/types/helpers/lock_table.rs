/// [*Reference*](https://learn.microsoft.com/en-us/windows/win32/msi/lockpermissions-table#table)
#[derive(Clone, Copy, Debug, PartialEq, strum::Display)]
pub enum LockTable {
    File,
    Registry,
    CreateFolder,
}

impl msi::ToValue for LockTable {
    fn to_value(&self) -> msi::Value {
        self.to_string().into()
    }
}
