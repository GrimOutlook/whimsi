/// I took some liberties to add some aliases for more sensible names but left the names of the
/// folders listed in the documentation as well.
///
/// [Reference](https://learn.microsoft.com/en-us/windows/win32/msi/property-reference)
#[derive(strum::Display, strum::EnumString)]
pub enum SystemFolder {
    /// Not technically a system folder but can be used. Indicates the root folder of the install.
    TARGETDIR,
    ProgramFiles64Folder,
    ProgramFilesFolder,
    ProgramMenuFolder,
    StartMenuFolder,
    StartupFolder,
    AppDataFolder,
    LocalAppDataFolder,
    /// Documents folder for the current user.
    PersonalFolder,
    /// Documents folder for the current user. Alias of `PersonalFolder`.
    DocumentsFolder,
    /// Pictures folder for the current user.
    MyPicturesFolder,
    /// Pictures folder for the current user. Alias of `MyPicturesFolder`.
    PicturesFolder,
}
