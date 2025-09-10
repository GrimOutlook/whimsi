use crate::define_identifier_generator;
use crate::define_specific_identifier;
use crate::define_specific_identifier_parsing;
use crate::int_val;
use crate::str_val;
use crate::tables::builder_list_entry::MsiBuilderListEntry;
use crate::tables::component::table::ComponentIdentifier;
use crate::tables::dao::IsDao;
use crate::types::column::formatted::Formatted;
use crate::types::column::identifier::Identifier;
use crate::types::column::reg_path::RegPath;
use crate::types::helpers::to_unique_msi_identifier::ToUniqueMsiIdentifier;

define_specific_identifier!(registry);
define_specific_identifier_parsing!(registry);
define_identifier_generator!(registry);

#[derive(Debug, Clone, PartialEq)]
pub struct RegistryDao {
    registry: RegistryIdentifier,
    root: i16,
    key: RegPath,
    name: Formatted,
    value: Formatted,
    component: ComponentIdentifier,
}

impl IsDao for RegistryDao {
    fn to_row(&self) -> Vec<msi::Value> {
        vec![
            str_val!(self.registry),
            int_val!(self.root),
            str_val!(self.key),
            str_val!(self.name),
            str_val!(self.value),
            str_val!(self.component),
        ]
    }
}

impl ToUniqueMsiIdentifier for RegistryDao {
    fn to_unique_msi_identifier(&self) -> Option<Identifier> {
        self.registry.to_unique_msi_identifier()
    }
}

impl MsiBuilderListEntry for RegistryDao {
    fn conflicts(&self, other: &Self) -> bool {
        self.registry == other.registry
    }
}
