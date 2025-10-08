use crate::{
    tables::{builder_list_entry::MsiBuilderListEntry, dao::IsDao},
    types::{
        column::{
            custom_source::CustomSource, formatted::Formatted,
            identifier::Identifier,
        },
        helpers::{
            to_msi_value::ToMsiOptionalValue,
            to_unique_msi_identifier::ToUniqueMsiIdentifier,
        },
    },
};

#[derive(Debug, Clone, PartialEq)]
pub struct CustomActionDao {
    action: Identifier,
    typ: i16,
    source: Option<CustomSource>,
    target: Option<Formatted>,
    extended_type: i32,
}

impl IsDao for CustomActionDao {
    fn to_row(&self) -> Vec<msi::Value> {
        vec![
            self.action.clone().into(),
            self.typ.into(),
            self.source.to_optional_value(),
            self.target.to_optional_value(),
            self.extended_type.into(),
        ]
    }
}

impl MsiBuilderListEntry for CustomActionDao {
    fn conflicts(&self, other: &Self) -> bool {
        self.action == other.action
    }
    // TODO: Overload the `.add()` function and make it fail if the user inputs an identifier that
    // shares a name with a StandardAction.
}

impl ToUniqueMsiIdentifier for CustomActionDao {
    fn to_unique_msi_identifier(&self) -> Option<Identifier> {
        None
    }
}
