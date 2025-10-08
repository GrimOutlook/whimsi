use anyhow::Context;
use getset::Getters;

use crate::tables::Table::Icon;
use crate::tables::builder_list_entry::MsiBuilderListEntry;
use crate::tables::component::table::ComponentIdentifier;
use crate::tables::dao::IsDao;
use crate::tables::directory::directory_identifier::DirectoryIdentifier;
use crate::tables::icon::table::IconIdentifier;
use crate::types::column::binary::Binary;
use crate::types::column::condition::Condition;
use crate::types::column::filename::Filename;
use crate::types::column::formatted::Formatted;
use crate::types::column::guid::Guid;
use crate::types::column::identifier::Identifier;
use crate::types::column::identifier::ToIdentifier;
use crate::types::helpers::to_msi_value::ToMsiOptionalValue;
use crate::types::helpers::to_unique_msi_identifier::ToUniqueMsiIdentifier;

#[derive(Debug, Clone, PartialEq, Getters, derive_more::Constructor)]
#[getset(get = "pub(crate)")]
pub struct IconDao {
    name: IconIdentifier,
    data: Binary,
}

impl IsDao for IconDao {
    fn to_row(&self) -> Vec<msi::Value> {
        vec![self.name.to_identifier().into(), self.data.clone().into()]
    }
}
impl MsiBuilderListEntry for IconDao {
    fn conflicts(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl ToUniqueMsiIdentifier for IconDao {
    fn to_unique_msi_identifier(&self) -> Option<Identifier> {
        self.name.to_unique_msi_identifier()
    }
}
