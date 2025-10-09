#[derive(
    Debug,
    Clone,
    PartialEq,
    Default,
    derive_more::From,
    derive_more::Display,
    whimsi_macros::IntoStrMsiValue,
)]
pub struct Formatted(String);

impl msi::ToValue for Formatted {
    fn to_value(&self) -> msi::Value {
        self.0.into()
    }
}
