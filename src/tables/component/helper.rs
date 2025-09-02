use derivative::Derivative;
use getset::Getters;
use uuid::Uuid;

use crate::types::column::{condition::Condition, identifier::Identifier};

use super::attributes::ComponentAttributes;

// NOTE: Keypath is not included as a property as that value will be determined by what contains
// the object.
#[derive(Clone, Debug, Derivative, Getters, PartialEq)]
#[get = "pub"]
#[derivative(Default)]
pub struct Component {
    /// Components without GUIDs will not be registerd as installed, meaning that it cannot be
    /// uninstalled and must be manually removed.
    guid: Option<Uuid>,
    feature: Option<Identifier>,

    #[derivative(Default(value = "0"))]
    attributes: i16,
    condition: Option<Condition>,
}
