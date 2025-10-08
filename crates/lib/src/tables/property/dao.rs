use crate::tables::builder_list_entry::MsiBuilderListEntry;
use crate::tables::dao::IsDao;
use crate::tables::property::property_text::PropertyText;
use crate::types::column::identifier::Identifier;
use crate::types::helpers::to_unique_msi_identifier::ToUniqueMsiIdentifier;

#[derive(Clone, Debug, Default, derive_more::Constructor)]
pub struct PropertyDao {
    property: Identifier,
    value: PropertyText,
}

impl IsDao for PropertyDao {
    fn to_row(&self) -> Vec<msi::Value> {
        vec![self.property.clone().into(), self.value.clone().into()]
    }
}

impl MsiBuilderListEntry for PropertyDao {
    fn conflicts(&self, other: &Self) -> bool {
        self.property == other.property
    }
}

impl ToUniqueMsiIdentifier for PropertyDao {
    fn to_unique_msi_identifier(&self) -> Option<Identifier> {
        Some(self.property.clone())
    }
}
