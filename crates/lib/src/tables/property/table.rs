use std::collections::HashMap;

use itertools::Itertools;

use crate::constants::*;
use crate::msi_list_boilerplate;
use crate::msi_table_boilerplate;
use crate::tables::builder_list::MsiBuilderList;
use crate::tables::builder_table::MsiBuilderTable;
use crate::tables::property::dao::PropertyDao;
use crate::types::column::identifier::Identifier;

#[derive(Clone, Debug, Default)]
pub struct PropertyTable {
    entries: Vec<PropertyDao>,
}

impl MsiBuilderTable for PropertyTable {
    type TableValue = PropertyDao;

    msi_table_boilerplate!();

    fn name(&self) -> &'static str {
        "Property"
    }

    fn columns(&self) -> Vec<msi::Column> {
        vec![
            msi::Column::build("Property")
                .primary_key()
                .id_string(DEFAULT_IDENTIFIER_MAX_LEN),
            msi::Column::build("Value")
                .localizable()
                .text_string(PROPERTY_VALUE_MAX_LEN),
        ]
    }
}

msi_list_boilerplate!(PropertyTable, PropertyDao);
