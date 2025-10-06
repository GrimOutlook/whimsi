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
            whimsi_msi::Column::build("MsiLockPermissionsEx")
                .primary_key()
                .text_string(255),
            whimsi_msi::Column::build("LockObject")
                .id_string(PROPERTY_VALUE_MAX_LEN),
            whimsi_msi::Column::build("Table").text_string(255),
            whimsi_msi::Column::build("SDDLText")
                .category(whimsi_msi::Category::FormattedSddlText)
                .string(255),
            whimsi_msi::Column::build("Condition")
                .nullable()
                .category(whimsi_msi::Category::Condition)
                .string(255),
        ]
    }
}

msi_list_boilerplate!(LockPermissionsTable, LockPermissionsDao);
