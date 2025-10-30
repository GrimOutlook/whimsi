use crate::tables::BinaryIdentifier;
use crate::tables::DirectoryIdentifier;
use crate::tables::FileIdentifier;
use crate::tables::PropertyIdentifier;
use crate::types::column::identifier::Identifier;
use crate::types::column::identifier::ToIdentifier;
use crate::types::column::identifier::ambassador_impl_ToIdentifier;

// TODO: The documentation seems to imply you can do something other than an
// external key but never explains what that would do.
// https://learn.microsoft.com/en-us/windows/win32/msi/customsource
// https://learn.microsoft.com/en-us/windows/win32/msi/customaction-table
#[derive(
    Debug,
    Clone,
    PartialEq,
    derive_more::Display,
    whimsi_macros::IdentifierToValue,
    ambassador::Delegate,
)]
#[delegate(ToIdentifier)]
pub enum CustomSource {
    Directory(DirectoryIdentifier),
    File(FileIdentifier),
    Binary(BinaryIdentifier),
    Property(PropertyIdentifier),
}
