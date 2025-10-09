/// [*Reference*](https://learn.microsoft.com/en-us/windows/win32/msi/shortcut-table#showcmd)
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ShowCmd {
    ShowNormal = 1,
    ShowMaximized = 3,
    ShowMinNoActive = 7,
}
