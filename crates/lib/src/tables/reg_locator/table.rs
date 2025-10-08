use crate::{
    constants::{DEFAULT_IDENTIFIER_MAX_LEN, REGPATH_MAX_LEN},
    msi_list_boilerplate, msi_table_boilerplate,
    tables::{
        builder_table::MsiBuilderTable, reg_locator::dao::RegLocatorDao,
        signature::table::SignatureIdentifier,
    },
    types::column::{formatted::Formatted, reg_path::RegPath},
};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct RegLocatorTable {
    entries: Vec<RegLocatorDao>,
}

msi_list_boilerplate!(RegLocatorTable, RegLocatorDao);

impl MsiBuilderTable for RegLocatorTable {
    type TableValue = RegLocatorDao;
    msi_table_boilerplate!();

    fn name(&self) -> &'static str {
        "RegLocator"
    }

    fn columns(&self) -> Vec<msi::Column> {
        vec![
            msi::Column::build("Signature_")
                .primary_key()
                .id_string(DEFAULT_IDENTIFIER_MAX_LEN),
            msi::Column::build("Root").int16(),
            msi::Column::build("Key")
                .category(msi::Category::RegPath)
                .string(REGPATH_MAX_LEN),
            msi::Column::build("Name").nullable().formatted_string(255),
            msi::Column::build("Type").nullable().int16(),
        ]
    }
}
