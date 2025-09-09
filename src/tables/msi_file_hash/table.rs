use crate::{
    constants::DEFAULT_IDENTIFIER_MAX_LEN,
    msi_list_boilerplate, msi_table_boilerplate,
    tables::{
        builder_list::MsiBuilderList, builder_table::MsiBuilderTable,
        msi_file_hash::dao::MsiFileHashDao,
    },
};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct MsiFileHashTable {
    entries: Vec<MsiFileHashDao>,
}

impl MsiBuilderList for MsiFileHashTable {
    type ListValue = MsiFileHashDao;

    msi_list_boilerplate!();
}

impl MsiBuilderTable for MsiFileHashTable {
    type TableValue = MsiFileHashDao;
    msi_table_boilerplate!();

    fn name(&self) -> &'static str {
        "MsiFileHash"
    }

    fn columns(&self) -> Vec<msi::Column> {
        vec![
            msi::Column::build("File_")
                .primary_key()
                .id_string(DEFAULT_IDENTIFIER_MAX_LEN),
            msi::Column::build("Options").int16(),
            msi::Column::build("HashPart1").int32(),
            msi::Column::build("HashPart2").int32(),
            msi::Column::build("HashPart3").int32(),
            msi::Column::build("HashPart4").int32(),
        ]
    }
}
