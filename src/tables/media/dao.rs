use anyhow::Context;
use getset::Getters;

use crate::int_val;
use crate::opt_str_val;
use crate::tables::builder_list_entry::MsiBuilderListEntry;
use crate::tables::dao::IsDao;
use crate::tables::media::cabinet_identifier::CabinetHandle;
use crate::tables::media::cabinet_identifier::CabinetIdentifier;
use crate::tables::media::disk_id::DiskId;
use crate::tables::media::disk_id::{self};
use crate::tables::media::last_sequence::LastSequence;
use crate::tables::media::property::Property;
use crate::types::column::identifier::Identifier;
use crate::types::column::sequence::IncludedSequence;
use crate::types::column::sequence::Sequence;
use crate::types::helpers::cabinet_info::CabinetInfo;
use crate::types::helpers::to_unique_msi_identifier::ToUniqueMsiIdentifier;

#[derive(Clone, Debug, Getters, PartialEq)]
#[getset(get = "pub")]
pub struct MediaDao {
    disk_id: DiskId,
    last_sequence: LastSequence,
    disk_prompt: Option<String>,
    /// This should only be `None` when referencing external media
    cabinet: Option<CabinetHandle>,
    volume_label: Option<String>,
    source: Option<Property>,
}

impl MediaDao {
    pub fn internal(
        disk_id: usize,
        cabinet_identifier: CabinetHandle,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            disk_id: DiskId::try_from(disk_id).with_context(|| {
                format!(
                    "Cannot create a MediaTable entry with disk ID [{disk_id}]"
                )
            })?,
            last_sequence: LastSequence::try_from(0)
                .expect("Hard-coded LastSequence of [0] is incorrect somehow?"),
            disk_prompt: None,
            cabinet: Some(cabinet_identifier),
            volume_label: None,
            source: None,
        })
    }

    pub fn cabinet_id(&self) -> Option<CabinetIdentifier> {
        self.cabinet.clone()?.try_as_internal()
    }

    pub fn set_last_sequence(
        &mut self,
        cab: &CabinetInfo,
    ) -> anyhow::Result<Sequence> {
        self.last_sequence = cab
            .files()
            .len()
            .try_into()
            .context("Cabinet has too many files to represent in the LastSequence column")?;
        Ok(Sequence::Included(IncludedSequence::new(self.last_sequence.into())))
    }
}

impl IsDao for MediaDao {
    fn to_row(&self) -> Vec<msi::Value> {
        vec![
            int_val!(self.disk_id),
            int_val!(self.last_sequence),
            opt_str_val!(self.disk_prompt),
            opt_str_val!(self.cabinet),
            opt_str_val!(self.volume_label),
            opt_str_val!(self.source),
        ]
    }
}

impl MsiBuilderListEntry for MediaDao {
    fn conflicts(&self, other: &Self) -> bool {
        self.disk_id == other.disk_id
    }
}

impl ToUniqueMsiIdentifier for MediaDao {
    fn to_unique_msi_identifier(&self) -> Option<Identifier> {
        None
    }
}
