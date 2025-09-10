use crate::constants::*;
use crate::types::column::sequence::IncludedSequence;
use crate::types::column::sequence::Sequence;
use crate::types::helpers::cabinet_info::CabinetInfo;

#[derive(Debug, Clone, Copy, derive_more::Into, PartialEq)]
pub struct LastSequence(i16);

macro_rules! try_from_integer {
    ($($t:ty),*) => ($(
        impl TryFrom<$t> for LastSequence {
            type Error = anyhow::Error;

            fn try_from(value: $t) -> anyhow::Result<Self> {
                // LastSequence must be greater than or equal to 0.
                // LastSequence must be less than 32767.
                // Numbers found [here](https://learn.microsoft.com/en-us/windows/win32/msi/media-table).
                // TODO: Create real errors
                anyhow::ensure!(value >= LAST_SEQUENCE_MIN.try_into().unwrap(), format!("LastSequence number [{}] must be greater than or equal to [{}]", value, LAST_SEQUENCE_MIN));
                anyhow::ensure!(value < LAST_SEQUENCE_MAX.try_into().unwrap(), format!("LastSequence number [{}] must be less than [{}]", value, LAST_SEQUENCE_MAX));
                let value = i16::try_from(value)?;
                Ok(LastSequence(value))
            }
        }
    )*)
}

try_from_integer!(i16, u16, i32, u32, i64, u64, isize, usize);

impl From<LastSequence> for msi::Value {
    fn from(value: LastSequence) -> Self {
        Into::<i16>::into(value).into()
    }
}
