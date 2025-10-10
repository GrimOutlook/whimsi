/// [*Reference*](https://learn.microsoft.com/en-us/windows/win32/msi/lockpermissions-table#table)
#[derive(
    Clone, Copy, Debug, PartialEq, strum::Display, whimsi_macros::StrToValue,
)]
pub enum LockTable {
    File,
    Registry,
    CreateFolder,
}
