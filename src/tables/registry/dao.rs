use crate::{
    define_identifier_generator, define_specific_identifier,
    define_specific_identifier_parsing, int_val, str_val,
    tables::{
        builder_list_entry::MsiBuilderListEntry,
        component::table::ComponentIdentifier, dao::IsDao,
    },
    types::column::{
        formatted::Formatted,
        identifier::{Identifier, ToOptionalIdentifier},
        reg_path::RegPath,
    },
};

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

impl ToOptionalIdentifier for RegistryDao {
    fn to_optional_identifier(&self) -> Option<Identifier> {
        self.registry.to_optional_identifier()
    }
}

impl MsiBuilderListEntry for RegistryDao {
    fn conflicts(&self, other: &Self) -> bool {
        self.registry == other.registry
    }
}
