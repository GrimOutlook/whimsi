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
