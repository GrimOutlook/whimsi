/// [*Reference*](https://learn.microsoft.com/en-us/windows/win32/msi/registry-table#root)
#[derive(Clone, Copy, Debug, PartialEq, whimsi_macros::ReprToValue)]
#[repr(i32)]
pub enum RegistryRoot {
    None         = -1,
    ClassesRoot  = 0,
    CurrentUser  = 1,
    LocalMachine = 2,
    Users        = 3,
}
