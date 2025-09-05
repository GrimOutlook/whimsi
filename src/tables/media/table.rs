use anyhow::ensure;

use crate::msitable_boilerplate;
use crate::tables::builder_table::MsiBuilderTable;
use crate::tables::media::dao::MediaDao;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct MediaTable(Vec<MediaDao>);

impl MediaTable {
    pub(crate) fn get_last_internal_media(&mut self) -> Option<&mut MediaDao> {
        // Since only internal cabinets have an ID we can just verify that the cabinet_id is
        // populated.
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
        vec![]
    }

    fn rows(&self) -> Vec<Vec<msi::Value>> {
        todo!()
    }

    fn contains(&self, dao: &MediaDao) -> bool {
        // NOTE: We purposefully allow entries that have the same DefaultDir and
        // are contained by the same parent because you can assign
        // different components to these entries if you want
        // both components to install to the same location but based on separate
        // criteria.
        self.0.iter().find(|entry| entry.disk_id() == dao.disk_id()).is_some()
    }

    fn add(&mut self, dao: Self::TableValue) -> anyhow::Result<()> {
        // TODO: Create actual error for disk ID collision.
        ensure!(!self.contains(&dao), "TEMPERROR");
        self.0.push(dao);
        Ok(())
    }
}
