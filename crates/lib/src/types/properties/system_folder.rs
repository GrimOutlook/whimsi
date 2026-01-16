// TODO: Look into enabling custom directories based on properties.
// I did about an hour of work before realizing it was more effort than I
// needed to use right now while just trying to get basic functionality up and
// running. From the little work I did I learned custom system folders will
// need to track their own parent, as it can either be None (meaning TARGETDIR)
// or it could be another directory already defined. It will also have to be
// verified that the identifier given for the parent (if not None) and the ID
// given for the new custom system directory is in the `Property` table
// beforehand as this is where the value for the directory Identifier will come
// from.

use itertools::Itertools;
use strum::IntoEnumIterator;
use thiserror::Error;

use crate::types::column::identifier::Identifier;
use crate::types::helpers::primary_identifier::PrimaryIdentifier;

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    PartialOrd,
    strum::Display,
    strum::EnumIter,
    Ord,
    Eq,
)]
pub enum SystemFolderProperty {
    /// The full path to the directory that contains administrative tools.
    AdminToolsFolder,
    /// The full path to the Roaming folder for the current user.
    AppDataFolder,
    /// The full path to application data for all users.
    CommonAppDataFolder,
    /// The full path to the predefined 64-bit Common Files folder.
    CommonFiles64Folder,
    /// The full path to the Common Files folder for the current user.
    CommonFilesFolder,
    /// The installer sets the DesktopFolder property to the full path of the
    /// current user's Desktop folder. If an "All Users" profile exists and the
    /// ALLUSERS property is set, then this property is set to the folder in
    /// the "All Users" profile.
    DesktopFolder,
    /// The full path to the Favorites folder for the current user.
    FavoritesFolder,
    /// The full path to the Fonts folder.
    FontsFolder,
    /// The full path to the folder that contains local (nonroaming)
    /// applications.
    LocalAppDataFolder,
    /// The full path to the Pictures folder.
    MyPicturesFolder,
    /// The full path to the NetHood folder.
    NetHoodFolder,
    /// The full path to the Documents folder for the current user.
    PersonalFolder,
    /// The full path to the PrintHood folder.
    PrintHoodFolder,
    /// The full path to the predefined 64-bit Program Files folder.
    ProgramFiles64Folder,
    /// The full path to the predefined 32-bit Program Files folder.
    ProgramFilesFolder,
    /// The full path to the Program Menu folder.
    ProgramMenuFolder,
    /// The full path to the Recent folder.
    RecentFolder,
    /// The full path to the SendTo folder for the current user.
    SendToFolder,
    /// The full path to the Start menu folder.
    StartMenuFolder,
    /// The full path to the Startup folder.
    StartupFolder,
    /// The full path to folder for 16-bit system DLLs.
    System16Folder,
    /// The full path to the predefined System64 folder.
    System64Folder,
    /// The full path to the System folder for the current user.
    SystemFolder,
    /// Specifies the root destination directory for the installation. During
    /// an administrative installation this property is the location to
    /// copy the installation package.
    // TODO: Move this to the System Properties enum where it belongs.
    TARGETDIR,
    /// The full path to the Temp folder.
    TempFolder,
    /// The full path to the Template folder for the current user.
    TemplateFolder,
    /// The full path to the Windows folder.
    WindowsFolder,
    /// The volume of the Windows folder.
    WindowsVolume,
}

impl SystemFolderProperty {
    pub fn from_identifier(identifier: &Identifier) -> anyhow::Result<Self> {
        identifier.clone().try_into()
    }
}

impl PartialEq<Identifier> for SystemFolderProperty {
    fn eq(&self, other: &Identifier) -> bool {
        other == &self.into()
    }
}

impl TryFrom<Identifier> for SystemFolderProperty {
    type Error = anyhow::Error;

    fn try_from(identifier: Identifier) -> Result<Self, Self::Error> {
        SystemFolderProperty::iter().find(|f| identifier == f.into()).ok_or(
            SystemFolderConversionError::InvalidSystemFolder { identifier }
                .into(),
        )
    }
}

impl PrimaryIdentifier for SystemFolderProperty {
    fn primary_identifier(&self) -> Option<Identifier> {
        Some(self.into())
    }
}

#[derive(Debug, Error)]
pub enum SystemFolderConversionError {
    #[error("Identifier {identifier} didn't match any known system folder")]
    InvalidSystemFolder { identifier: Identifier },
}
