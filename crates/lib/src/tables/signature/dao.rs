use crate::constants::*;
use crate::constants::{self};
use crate::tables::builder_list_entry::MsiBuilderListEntry;
use crate::tables::dao::IsDao;
use crate::tables::signature::table::SignatureIdentifier;
use crate::types::column::condition::Condition;
use crate::types::column::identifier::{Identifier, ToIdentifier};
use crate::types::column::sequence::Sequence;
use crate::types::helpers::to_msi_value::ToMsiOptionalValue;
use crate::types::helpers::to_unique_msi_identifier::ToUniqueMsiIdentifier;
use crate::types::standard_action::AdvtAction;

#[derive(Debug, Clone, PartialEq)]
pub struct SignatureDao {
    signature: SignatureIdentifier,
    filename: String,
    min_version: Option<String>,
    max_version: Option<String>,
    min_size: Option<i32>,
    max_size: Option<i32>,
    min_date: Option<i32>,
    max_date: Option<i32>,
    languages: Option<String>,
}

impl IsDao for SignatureDao {
    fn to_row(&self) -> Vec<msi::Value> {
        vec![
            self.signature.clone().into(),
            self.filename.clone().into(),
            self.min_version.to_optional_value(),
            self.max_version.to_optional_value(),
            self.min_size.to_optional_value(),
            self.max_size.to_optional_value(),
            self.min_date.to_optional_value(),
            self.max_date.to_optional_value(),
            self.languages.to_optional_value(),
        ]
    }
}

impl ToUniqueMsiIdentifier for SignatureDao {
    fn to_unique_msi_identifier(&self) -> Option<Identifier> {
        self.signature.to_unique_msi_identifier()
    }
}

impl MsiBuilderListEntry for SignatureDao {
    fn conflicts(&self, other: &Self) -> bool {
        self.signature == other.signature
    }
}
