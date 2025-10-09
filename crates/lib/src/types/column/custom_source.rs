use crate::types::column::identifier::ambassador_impl_ToIdentifier;
use crate::{
    tables::{
        BinaryIdentifier, DirectoryIdentifier, FileIdentifier,
        PropertyIdentifier,
    },
    types::column::identifier::{Identifier, ToIdentifier},
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
    ambassador::Delegate,
)]
#[delegate(ToIdentifier)]
pub enum CustomSource {
    Directory(DirectoryIdentifier),
    File(FileIdentifier),
    Binary(BinaryIdentifier),
    Property(PropertyIdentifier),
}

impl msi::ToValue for CustomSource {
    fn to_value(&self) -> msi::Value {
        self.to_identifier().into()
    }
}
