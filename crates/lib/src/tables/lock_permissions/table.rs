use std::collections::HashMap;

use itertools::Itertools;

use crate::constants::*;
use crate::msi_list_boilerplate;
use crate::msi_table_boilerplate;
use crate::tables::builder_list::MsiBuilderList;
use crate::tables::builder_table::MsiBuilderTable;
use crate::tables::lock_permissions::dao::LockPermissionsDao;
use crate::tables::property::dao::PropertyDao;
use crate::types::column::identifier::Identifier;

#[derive(Clone, Debug, Default)]
pub struct LockPermissionsTable {
    entries: Vec<LockPermissionsDao>,
}

impl MsiBuilderTable for LockPermissionsTable {
    type TableValue = LockPermissionsDao;

    msi_table_boilerplate!();

    fn name(&self) -> &'static str {
        "LockPermissions"
    }

    fn columns(&self) -> Vec<whimsi_msi::Column> {
        vec![
            whimsi_msi::Column::build("LockObject").primary_key().id_string(72),
            whimsi_msi::Column::build("Table").text_string(255),
            whimsi_msi::Column::build("Domain")
                .nullable()
                .category(whimsi_msi::Category::Formatted)
                .string(255),
            whimsi_msi::Column::build("User")
                .category(whimsi_msi::Category::Formatted)
                .string(255),
            whimsi_msi::Column::build("Permission").nullable().int32(),
        ]
    }
}

msi_list_boilerplate!(LockPermissionsTable, LockPermissionsDao);
