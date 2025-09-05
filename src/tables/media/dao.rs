use anyhow::Context;
use getset::Getters;

use crate::tables::media::cabinet_identifier::CabinetIdentifier;
use crate::tables::media::disk_id::{self, DiskId};
use crate::tables::media::last_sequence::LastSequence;
use crate::tables::media::property::Property;
use crate::types::column::identifier::Identifier;
use crate::types::column::sequence::{IncludedSequence, Sequence};
use crate::types::helpers::cabinet_info::CabinetInfo;

#[derive(Clone, Debug, Getters, PartialEq)]
#[getset(get = "pub")]
pub struct MediaDao {
    disk_id: DiskId,
    last_sequence: LastSequence,
    disk_prompt: Option<String>,
    /// This should only be `None` when referencing external media
    cabinet: Option<CabinetIdentifier>,
    volume_label: Option<String>,
    source: Option<Property>,
}

impl MediaDao {
    pub fn internal(
        disk_id: usize,
        cabinet_identifier: CabinetIdentifier,
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

    pub fn cabinet_id(&self) -> Option<Identifier> {
        self.cabinet.clone()?.into_identifier()
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
        Ok(Sequence::Included(IncludedSequence::new(
            self.last_sequence.into(),
            cab.id().clone(),
        )))
    }
}
