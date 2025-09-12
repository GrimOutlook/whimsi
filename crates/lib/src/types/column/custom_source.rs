use crate::{
    tables::{
        binary::table::BinaryIdentifier,
        directory::directory_identifier::DirectoryIdentifier,
        file::table::FileIdentifier,
    },
    types::column::identifier::Identifier,
};

// TODO: The documentation seems to imply you can do something other than an external key but never
// explains what that would do.
// https://learn.microsoft.com/en-us/windows/win32/msi/customsource
// https://learn.microsoft.com/en-us/windows/win32/msi/customaction-table
#[derive(
    Debug,
    Clone,
    PartialEq,
    derive_more::Display,
    whimsi_macros::IntoStrMsiValue,
)]
pub enum CustomSource {
    Directory(DirectoryIdentifier),
    File(FileIdentifier),
    Binary(BinaryIdentifier),
    Property(Identifier),
}
