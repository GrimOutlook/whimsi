use bitflags::Flags;

use crate::define_identifier_generator;
use crate::define_specific_identifier;
use crate::define_specific_identifier_parsing;
use crate::tables::builder_list_entry::MsiBuilderListEntry;
use crate::tables::component::table::ComponentIdentifier;
use crate::tables::dao::IsDao;
use crate::tables::service_control::event::Event;
use crate::tables::service_install::table::ServiceInstallIdentifier;
use crate::types::column::formatted::Formatted;
use crate::types::column::identifier::Identifier;
use crate::types::column::identifier::ToIdentifier;
use crate::types::helpers::to_msi_value::ToMsiOptionalValue;
use crate::types::helpers::to_unique_msi_identifier::ToUniqueMsiIdentifier;

define_specific_identifier!(ServiceControl);
define_specific_identifier_parsing!(ServiceControl);
define_identifier_generator!(ServiceControl);

#[derive(Debug, Clone, PartialEq)]
pub struct ServiceControlDao {
    service_control: ServiceControlIdentifier,
    name: Formatted,
    event: Event,
    arguments: Option<Formatted>,
    wait: Option<i16>,
    component_: ComponentIdentifier,
}
impl ServiceControlDao {
    pub fn new(
        identifier: ServiceControlIdentifier,
        name: Formatted,
        event: Event,
        wait: bool,
        component_id: ComponentIdentifier,
    ) -> ServiceControlDao {
        ServiceControlDao {
            service_control: identifier,
            name,
            event,
            arguments: None,
            wait: if (wait) { Some(1) } else { None },
            component_: component_id,
        }
    }
}

impl IsDao for ServiceControlDao {
    fn to_row(&self) -> Vec<whimsi_msi::Value> {
        vec![
            self.service_control.to_identifier().into(),
            self.name.clone().into(),
            whimsi_msi::Value::Int(self.event.bits() as i32),
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
