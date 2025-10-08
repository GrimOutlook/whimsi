use anyhow::ensure;
use itertools::Itertools;

use super::dao::IconDao;
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

define_specific_identifier!(Icon);
define_specific_identifier_parsing!(Icon);
define_identifier_generator!(Icon);
define_generator_table!(
    Icon,
    vec![
        msi::Column::build("Name")
            .primary_key()
            .id_string(DEFAULT_IDENTIFIER_MAX_LEN),
        msi::Column::build("Data").binary()
    ]
);

msi_list_boilerplate!(IconTable, IconDao);
implement_id_generator_for_table!(IconTable, IconIdGenerator);
implement_new_for_id_generator_table!(IconTable, IconIdGenerator);
