use crate::types::standard_action::StandardAction;

#[derive(
    Debug,
    Clone,
    PartialEq,
    derive_more::Display,
    derive_more::From,
    whimsi_macros::IntoStrMsiValue,
)]
pub enum ActionIdentifier {
    StandardAction(StandardAction),
    CustomAction,
}

impl msi::ToValue for ActionIdentifier {
    fn to_value(&self) -> msi::Value {
        self.to_string().into()
    }
}
