use anyhow::ensure;

use crate::constants::*;
use crate::msitable_boilerplate;
use crate::tables::builder_table::MsiBuilderTable;
use crate::tables::media::dao::MediaDao;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct MediaTable(Vec<MediaDao>);

impl MediaTable {
    pub(crate) fn get_last_internal_media_mut(
        &mut self,
    ) -> Option<&mut MediaDao> {
        // Since only internal cabinets have an ID we can just verify that the
        // cabinet_id is populated.
        self.0.iter_mut().rfind(|media| media.cabinet_id().is_some())
    }
}

impl MsiBuilderTable for MediaTable {
    type TableValue = MediaDao;

    // Boilderplate needed to access information on the inner object
    msitable_boilerplate!();

    fn name(&self) -> &'static str {
        "Media"
    }

    fn columns(&self) -> Vec<msi::Column> {
        vec![
            msi::Column::build("DiskId").primary_key().int16(),
            msi::Column::build("LastSequence").int16(),
            msi::Column::build("DiskPrompt")
                .nullable()
                .text_string(DISK_PROMPT_MAX_LEN),
            msi::Column::build("Cabinet")
                .nullable()
                .category(msi::Category::Cabinet)
                .string(CABINET_MAX_LEN),
            msi::Column::build("VolumeLabel")
                .nullable()
                .text_string(VOLUME_LABEL_MAX_LEN),
            msi::Column::build("Source")
                .nullable()
                .category(msi::Category::Property)
                .string(SOURCE_MAX_LEN),
        ]
    }
}
