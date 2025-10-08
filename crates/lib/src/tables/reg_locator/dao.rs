use crate::{
    tables::{
        builder_list_entry::MsiBuilderListEntry, dao::IsDao,
        signature::table::SignatureIdentifier,
    },
    types::{
        column::{
            formatted::Formatted, identifier::Identifier, reg_path::RegPath,
        },
        helpers::{
            to_msi_value::ToMsiOptionalValue,
            to_unique_msi_identifier::ToUniqueMsiIdentifier,
        },
    },
};

#[derive(Debug, Clone, PartialEq)]
pub struct RegLocatorDao {
    signature: SignatureIdentifier,
    root: i16,
    key: RegPath,
    name: Option<Formatted>,
    typ: Option<i16>,
}

impl ToUniqueMsiIdentifier for RegLocatorDao {
    fn to_unique_msi_identifier(&self) -> Option<Identifier> {
        None
    }
}

impl IsDao for RegLocatorDao {
    fn to_row(&self) -> Vec<msi::Value> {
        vec![
            self.signature.clone().into(),
            self.root.into(),
            self.key.clone().into(),
            self.name.to_optional_value(),
            self.typ.to_optional_value(),
        ]
    }
}

impl MsiBuilderListEntry for RegLocatorDao {
    fn conflicts(&self, other: &Self) -> bool {
        self.signature == other.signature
    }
}
