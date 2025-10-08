use crate::{
    tables::{
        builder_list_entry::MsiBuilderListEntry, dao::IsDao,
        signature::table::SignatureIdentifier,
    },
    types::{
        column::identifier::Identifier,
        helpers::to_unique_msi_identifier::ToUniqueMsiIdentifier,
    },
};

#[derive(Debug, Clone, PartialEq)]
pub struct AppSearchDao {
    property: Identifier,
    signature: SignatureIdentifier,
}

impl IsDao for AppSearchDao {
    fn to_row(&self) -> Vec<msi::Value> {
        vec![self.property.clone().into(), self.signature.clone().into()]
    }
}

impl ToUniqueMsiIdentifier for AppSearchDao {
    fn to_unique_msi_identifier(&self) -> Option<Identifier> {
        None
    }
}

impl MsiBuilderListEntry for AppSearchDao {
    fn conflicts(&self, other: &Self) -> bool {
        self.property == other.property
    }
}
