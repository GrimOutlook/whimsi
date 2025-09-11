use crate::{
    constants::DEFAULT_IDENTIFIER_MAX_LEN, define_generator_table,
    define_identifier_generator, define_specific_identifier,
    define_specific_identifier_parsing, msi_list_boilerplate,
    tables::binary::dao::BinaryDao,
};

define_specific_identifier!(Binary);
define_specific_identifier_parsing!(Binary);
define_identifier_generator!(Binary);
msi_list_boilerplate!(BinaryTable, BinaryDao);
define_generator_table!(
    Binary,
    vec![
        whimsi_msi::Column::build("Name")
            .primary_key()
            .id_string(DEFAULT_IDENTIFIER_MAX_LEN),
        whimsi_msi::Column::build("Data").binary()
    ]
);
impl Default for BinaryTable {
    fn default() -> Self {
        Self { entries: Default::default(), generator: Default::default() }
    }
}
