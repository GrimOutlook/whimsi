use crate::{
    str_val,
    tables::{
        builder_list_entry::MsiBuilderListEntry, dao::IsDao,
        property::property_text::PropertyText,
    },
    types::column::identifier::{Identifier, ToOptionalIdentifier},
};

#[derive(Clone, Debug, Default)]
pub struct PropertyDao {
    property: Identifier,
    value: PropertyText,
}

impl IsDao for PropertyDao {
    fn to_row(&self) -> Vec<msi::Value> {
        vec![str_val!(self.property), str_val!(self.value)]
    }
}

impl MsiBuilderListEntry for PropertyDao {
    fn conflicts(&self, other: &Self) -> bool {
        self.property == other.property
    }
}

impl ToOptionalIdentifier for PropertyDao {
    fn to_optional_identifier(&self) -> Option<Identifier> {
        Some(self.property.clone())
    }
}
