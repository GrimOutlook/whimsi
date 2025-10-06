use bitflags::Flags;

use crate::{
    tables::{
        builder_list_entry::MsiBuilderListEntry,
        component::table::ComponentIdentifier, dao::IsDao,
        service_control::event::Event,
        service_install::table::ServiceInstallIdentifier,
    },
    types::{
        column::{
            formatted::Formatted,
            identifier::{Identifier, ToIdentifier},
        },
        helpers::{
            to_msi_value::ToMsiOptionalValue,
            to_unique_msi_identifier::ToUniqueMsiIdentifier,
        },
    },
};

#[derive(Debug, Clone)]
pub struct ServiceControlDao {
    service_control: ServiceInstallIdentifier,
    name: Formatted,
    event: Event,
    arguments: Option<Formatted>,
    wait: Option<i16>,
    component_: ComponentIdentifier,
}

impl IsDao for ServiceControlDao {
    fn to_row(&self) -> Vec<whimsi_msi::Value> {
        vec![
            self.service_control.to_identifier().into(),
            self.name.clone().into(),
            self.event.bits().into(),
            self.arguments.to_optional_value(),
            self.wait.to_optional_value(),
            self.component_.to_identifier().into(),
        ]
    }
}

impl MsiBuilderListEntry for ServiceControlDao {
    fn conflicts(&self, other: &Self) -> bool {
        self.service_control == other.service_control
    }
}

impl ToUniqueMsiIdentifier for ServiceControlDao {
    fn to_unique_msi_identifier(&self) -> Option<Identifier> {
        None
    }
}
