use crate::constants::DEFAULT_IDENTIFIER_MAX_LEN;
use crate::msi_list_boilerplate;
use crate::msi_table_boilerplate;
use crate::tables::builder_list::MsiBuilderList;
use crate::tables::builder_table::MsiBuilderTable;
use crate::tables::msi_file_hash::dao::MsiFileHashDao;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct MsiFileHashTable {
    entries: Vec<MsiFileHashDao>,
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

msi_list_boilerplate!(MsiFileHashTable, MsiFileHashDao);
