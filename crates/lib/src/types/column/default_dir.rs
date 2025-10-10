use super::identifier::Identifier;
use crate::types::column::filename::Filename;

#[derive(
    Clone,
    Debug,
    derive_more::Display,
    derive_more::From,
    PartialEq,
    whimsi_macros::StrToValue,
)]
pub enum DefaultDir {
    Identifier(Identifier),
    Filename(Filename),
}
