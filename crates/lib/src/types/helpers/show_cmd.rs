/// [*Reference*](https://learn.microsoft.com/en-us/windows/win32/msi/shortcut-table#showcmd)
#[derive(Clone, Copy, Debug, PartialEq, whimsi_macros::IntToValue)]
#[repr(i16)]
pub enum ShowCmd {
    ShowNormal = 1,
    ShowMaximized = 3,
    ShowMinNoActive = 7,
}
