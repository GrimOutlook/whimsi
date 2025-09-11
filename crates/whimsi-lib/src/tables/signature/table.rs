use itertools::Itertools;

use crate::constants::*;
use crate::define_generator_table;
use crate::define_identifier_generator;
use crate::define_specific_identifier;
use crate::define_specific_identifier_parsing;
use crate::msi_list_boilerplate;
use crate::msi_table_boilerplate;
use crate::tables::builder_list::MsiBuilderList;
use crate::tables::builder_table::MsiBuilderTable;
use crate::tables::signature::dao::SignatureDao;
use crate::types::standard_action::AdvtAction;

define_specific_identifier!(Signature);
define_specific_identifier_parsing!(Signature);
define_identifier_generator!(Signature);
define_generator_table!(
    Signature,
    vec![
        whimsi_msi::Column::build("Signature")
            .primary_key()
            .id_string(DEFAULT_IDENTIFIER_MAX_LEN),
        whimsi_msi::Column::build("FileName").text_string(FILENAME_MAX_LEN),
        // TODO: Fix these later when I'm not trying to get things working.
        whimsi_msi::Column::build("MinVersion").nullable().text_string(20),
        whimsi_msi::Column::build("MaxVersion").nullable().text_string(20),
        whimsi_msi::Column::build("MinSize").nullable().int32(),
        whimsi_msi::Column::build("MaxSize").nullable().int32(),
        whimsi_msi::Column::build("MinDate").nullable().int32(),
        whimsi_msi::Column::build("MaxDate").nullable().int32(),
        whimsi_msi::Column::build("Language").nullable().text_string(255),
    ]
);

// TODO: Figure out how to get rid of this when using the macro
impl Default for SignatureTable {
    fn default() -> Self {
        Self { entries: Default::default(), generator: Default::default() }
    }
}

msi_list_boilerplate!(SignatureTable, SignatureDao);
