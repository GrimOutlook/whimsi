use super::identifier::Identifier;
use crate::types::column::filename::Filename;
use crate::types::helpers::to_msi_value::IntoMsiValue;
use crate::types::helpers::to_msi_value::ambassador_impl_IntoMsiValue;

#[derive(
    Clone,
    Debug,
    derive_more::Display,
    derive_more::From,
    PartialEq,
    ambassador::Delegate,
)]
#[delegate(IntoMsiValue)]
pub enum DefaultDir {
    Identifier(Identifier),
    Filename(Filename),
}
