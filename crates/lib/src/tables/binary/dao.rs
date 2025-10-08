use crate::{
    tables::{
        binary::table::BinaryIdentifier,
        builder_list_entry::MsiBuilderListEntry, dao::IsDao,
    },
    types::{
        column::identifier::Identifier,
        helpers::to_unique_msi_identifier::ToUniqueMsiIdentifier,
    },
};

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryDao {
    name: BinaryIdentifier,
    data: Vec<u8>,
}

impl ToUniqueMsiIdentifier for BinaryDao {
    fn to_unique_msi_identifier(&self) -> Option<Identifier> {
        None
    }
}

impl MsiBuilderListEntry for BinaryDao {
    fn conflicts(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl IsDao for BinaryDao {
    fn to_row(&self) -> Vec<msi::Value> {
        vec![
            self.name.clone().into(),
            // TODO: It appears there is literally no way to write binary data to a table in the
            // current `msi` crate version.
            todo!(),
        ]
    }
}
