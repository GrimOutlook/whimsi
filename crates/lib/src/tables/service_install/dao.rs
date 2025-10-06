use anyhow::Context;
use getset::Getters;

use crate::tables::builder_list_entry::MsiBuilderListEntry;
use crate::tables::component::table::ComponentIdentifier;
use crate::tables::dao::IsDao;
use crate::tables::service_install::error_control::ErrorControl;
use crate::tables::service_install::service_type::ServiceType;
use crate::tables::service_install::start_type::StartType;
use crate::tables::service_install::table::ServiceInstallIdentifier;
use crate::types::column::condition::Condition;
use crate::types::column::formatted::Formatted;
use crate::types::column::guid::Guid;
use crate::types::column::identifier::Identifier;
use crate::types::column::identifier::ToIdentifier;
use crate::types::helpers::to_msi_value::ToMsiOptionalValue;
use crate::types::helpers::to_unique_msi_identifier::ToUniqueMsiIdentifier;

#[derive(Debug, Clone, PartialEq, Getters)]
#[getset(get = "pub(crate)")]
pub struct ServiceInstallDao {
    identifier: ServiceInstallIdentifier,
    name: Formatted,
    display_name: Option<Formatted>,
    service_type: ServiceType,
    start_type: StartType,
    error_control: ErrorControl,
    load_order_group: Option<Formatted>,
    dependencies: Option<Formatted>,
    start_name: Option<Formatted>,
    password: Option<Formatted>,
    arguments: Option<Formatted>,
    component_: ComponentIdentifier,
    description: Option<Formatted>,
}

impl ServiceInstallDao {
    pub fn new(
        identifier: ServiceInstallIdentifier,
        name: Formatted,
        service_type: ServiceType,
        start_type: StartType,
        error_control: ErrorControl,
        component_id: ComponentIdentifier,
    ) -> ServiceInstallDao {
        ServiceInstallDao {
            identifier,
            name,
            display_name: None,
            service_type,
            start_type,
            error_control,
            load_order_group: None,
            dependencies: None,
            start_name: None,
            password: None,
            arguments: None,
            component_: component_id,
            description: None,
        }
    }
}

impl IsDao for ServiceInstallDao {
    fn to_row(&self) -> Vec<whimsi_msi::Value> {
        vec![
            self.identifier.to_identifier().into(),
            self.name.clone().into(),
            self.display_name.to_optional_value(),
            (self.service_type as i16).into(),
            (self.start_type as i16).into(),
            (self.error_control as i16).into(),
            self.load_order_group.to_optional_value(),
            self.dependencies.to_optional_value(),
            self.start_name.to_optional_value(),
            self.password.to_optional_value(),
            self.arguments.to_optional_value(),
            self.component_.to_identifier().into(),
            self.description.to_optional_value(),
        ]
    }
}
impl MsiBuilderListEntry for ServiceInstallDao {
    fn conflicts(&self, other: &Self) -> bool {
        self.identifier == other.identifier
    }
}

impl ToUniqueMsiIdentifier for ServiceInstallDao {
    fn to_unique_msi_identifier(&self) -> Option<Identifier> {
        self.identifier.to_unique_msi_identifier()
    }
}
