#[derive(
    Debug,
    Clone,
    PartialEq,
    Default,
    derive_more::From,
    derive_more::Display,
    whimsi_macros::StrToValue,
)]
pub struct Formatted(String);
