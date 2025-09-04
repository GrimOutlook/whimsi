use crate::types::column::identifier::Identifier;
use crate::types::column::identifier::ToIdentifier;
use crate::types::column::identifier::ambassador_impl_ToIdentifier;
use crate::types::properties::system_folder::SystemFolder;

#[derive(
    Debug,
    Clone,
    PartialEq,
    ambassador::Delegate,
    derive_more::Display,
    derive_more::From,
)]
#[delegate(ToIdentifier)]
pub enum DirectoryIdentifier {
    SystemFolder(SystemFolder),
    Identifier(Identifier),
}
