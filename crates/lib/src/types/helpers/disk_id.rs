use crate::constants::*;

#[derive(
    Clone, Copy, Debug, PartialEq, derive_more::Display, derive_more::Into,
)]
pub struct DiskId(i16);

macro_rules! try_from_integer {
    ($($t:ty),*) => ($(
        impl TryFrom<$t> for DiskId {
            type Error = anyhow::Error;

            fn try_from(value: $t) -> anyhow::Result<Self> {
                // Numbers found [here](https://learn.microsoft.com/en-us/windows/win32/msi/media-table).
                // TODO: Create real errors
                anyhow::ensure!(value >= DISK_ID_MIN.try_into().unwrap(), format!("DiskId [{}] must be greater than [{}]", value, DISK_ID_MIN));
                let value = i16::try_from(value)?;
                Ok(DiskId(value))
            }
        }
    )*)
}

try_from_integer!(i8, u8, i16, u16, i32, u32, i64, u64, isize, usize);
impl msi::ToValue for DiskId {
    fn to_value(&self) -> msi::Value {
        self.0.into()
    }
}
