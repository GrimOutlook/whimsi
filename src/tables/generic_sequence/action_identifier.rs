use crate::types::standard_action::StandardAction;

#[derive(Debug, Clone, PartialEq, derive_more::Display)]
pub enum ActionIdentifier {
    StandardAction(StandardAction),
    CustomAction,
}
