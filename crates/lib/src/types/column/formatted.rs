#[derive(
    Debug,
    Clone,
    PartialEq,
    derive_more::From,
    derive_more::Display,
    whimsi_macros::IntoStrMsiValue,
)]
pub struct Formatted(String);
