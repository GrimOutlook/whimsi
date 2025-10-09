/// [*Reference*](https://learn.microsoft.com/en-us/windows/win32/msi/registry-table#root)
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RegistryRoot {
    None = -1,
    ClassesRoot = 0,
    CurrentUser = 1,
    LocalMachine = 2,
    Users = 3,
}
