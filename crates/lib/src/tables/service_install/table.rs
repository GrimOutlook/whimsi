use anyhow::ensure;
use itertools::Itertools;

use super::dao::ServiceInstallDao;
use crate::constants::*;
use crate::define_generator_table;
use crate::define_identifier_generator;
use crate::define_specific_identifier;
use crate::define_specific_identifier_parsing;
use crate::implement_id_generator_for_table;
use crate::implement_new_for_id_generator_table;
use crate::msi_list_boilerplate;
use crate::msi_table_boilerplate;
use crate::tables::builder_list::MsiBuilderList;
use crate::tables::builder_table::MsiBuilderTable;
use crate::tables::dao::IsDao;
use crate::types::column::identifier::Identifier;
use crate::types::column::identifier::ToIdentifier;

define_specific_identifier!(ServiceInstall);
define_specific_identifier_parsing!(ServiceInstall);
define_identifier_generator!(ServiceInstall);
define_generator_table!(
    ServiceInstall,
    vec![
        msi::Column::build("ServiceInstall")
            .primary_key()
            .id_string(DEFAULT_IDENTIFIER_MAX_LEN),
        msi::Column::build("Name").formatted_string(255),
        msi::Column::build("DisplayName")
            .nullable()
            .formatted_string(255),
        msi::Column::build("ServiceType").int32(),
        msi::Column::build("StartType").int32(),
        msi::Column::build("ErrorControl").int32(),
        msi::Column::build("LoadOrderGroup")
            .nullable()
            .formatted_string(255),
        msi::Column::build("Dependencies")
            .nullable()
            .formatted_string(255),
        msi::Column::build("StartName").nullable().formatted_string(255),
        msi::Column::build("Password").nullable().formatted_string(255),
        msi::Column::build("Arguments").nullable().formatted_string(255),
        msi::Column::build("Component_")
            .id_string(DEFAULT_IDENTIFIER_MAX_LEN),
        msi::Column::build("Description")
            .nullable()
            .formatted_string(255),
    ]
);

msi_list_boilerplate!(ServiceInstallTable, ServiceInstallDao);
implement_id_generator_for_table!(
    ServiceInstallTable,
    ServiceInstallIdGenerator
);
implement_new_for_id_generator_table!(
    ServiceInstallTable,
    ServiceInstallIdGenerator
);
