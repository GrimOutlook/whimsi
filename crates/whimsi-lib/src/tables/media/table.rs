use anyhow::ensure;

use crate::constants::*;
use crate::define_identifier_generator;
use crate::define_specific_identifier;
use crate::define_specific_identifier_parsing;
use crate::msi_list_boilerplate;
use crate::msi_table_boilerplate;
use crate::tables::builder_list::MsiBuilderList;
use crate::tables::builder_table::MsiBuilderTable;
use crate::tables::media::dao::MediaDao;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct MediaTable {
    entries: Vec<MediaDao>,
}

impl MediaTable {
    pub(crate) fn get_last_internal_media_mut(
        &mut self,
    ) -> Option<&mut MediaDao> {
        // Since only internal cabinets have an ID we can just verify that the
        // cabinet_id is populated.
        self.entries.iter_mut().rfind(|media| media.cabinet_id().is_some())
    }
}

impl MsiBuilderTable for MediaTable {
    type TableValue = MediaDao;

    // Boilderplate needed to access information on the inner object
    msi_table_boilerplate!();

    fn name(&self) -> &'static str {
        "Media"
    }

    fn columns(&self) -> Vec<whimsi_msi::Column> {
        vec![
            whimsi_msi::Column::build("DiskId").primary_key().int16(),
            whimsi_msi::Column::build("LastSequence").int16(),
            whimsi_msi::Column::build("DiskPrompt")
                .nullable()
                .text_string(DISK_PROMPT_MAX_LEN),
            whimsi_msi::Column::build("Cabinet")
                .nullable()
                .category(whimsi_msi::Category::Cabinet)
                .string(CABINET_MAX_LEN),
            whimsi_msi::Column::build("VolumeLabel")
                .nullable()
                .text_string(VOLUME_LABEL_MAX_LEN),
            whimsi_msi::Column::build("Source")
                .nullable()
                .category(whimsi_msi::Category::Property)
                .string(SOURCE_MAX_LEN),
        ]
    }
}

msi_list_boilerplate!(MediaTable, MediaDao);
