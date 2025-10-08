use anyhow::ensure;
use itertools::Itertools;

use super::dao::ShortcutDao;
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

define_specific_identifier!(Shortcut);
define_specific_identifier_parsing!(Shortcut);
define_identifier_generator!(Shortcut);
define_generator_table!(
    Shortcut,
    vec![
        msi::Column::build("Shortcut")
            .primary_key()
            .id_string(DEFAULT_IDENTIFIER_MAX_LEN),
        msi::Column::build("Directory_")
            .id_string(DEFAULT_IDENTIFIER_MAX_LEN),
        msi::Column::build("Name")
            .localizable()
            .category(msi::Category::Filename)
            .string(255),
        msi::Column::build("Component_")
            .id_string(DEFAULT_IDENTIFIER_MAX_LEN),
        msi::Column::build("Target")
            .category(msi::Category::Shortcut)
            .string(255),
        msi::Column::build("Arguments").nullable().formatted_string(255),
        msi::Column::build("Description")
            .nullable()
            .formatted_string(255),
        msi::Column::build("Hotkey").nullable().int16(),
        msi::Column::build("Icon_")
            .nullable()
            .id_string(DEFAULT_IDENTIFIER_MAX_LEN),
        msi::Column::build("IconIndex").nullable().int16(),
        msi::Column::build("ShowCmd").nullable().int16(),
        msi::Column::build("WkDir")
            .nullable()
            .id_string(DEFAULT_IDENTIFIER_MAX_LEN),
        msi::Column::build("DisplayResourceDLL")
            .nullable()
            .formatted_string(255),
        msi::Column::build("DisplayResourceId").nullable().int16(),
        msi::Column::build("DescriptionResourceDLL")
            .nullable()
            .formatted_string(255),
        msi::Column::build("DescriptionResourceId").nullable().int16(),
    ]
);

msi_list_boilerplate!(ShortcutTable, ShortcutDao);
implement_id_generator_for_table!(ShortcutTable, ShortcutIdGenerator);
implement_new_for_id_generator_table!(ShortcutTable, ShortcutIdGenerator);
