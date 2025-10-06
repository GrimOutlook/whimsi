/// Denotes the version of Windows Installer to use.
///
/// Don't ask me why it's called page_count. Microsoft doesn't make any sense to me either.
///
/// [Documentation](https://learn.microsoft.com/en-us/windows/win32/msi/page-count-summary)

pub type PageCount = MinimumWindowsInstallerVersion;
pub enum MinimumWindowsInstallerVersion {
    _2_0 = 200,
    _3_0 = 300,
    _3_1 = 301,
    _4_0 = 400,
    _4_5 = 405,
    _5_0 = 500,
}
