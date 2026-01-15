use nutype_macros::nutype;

use crate::constants::*;
use crate::types::helpers::to_msi_value::IntoMsiValue;

#[nutype(
    validate(greater_or_equal = DISK_ID_MIN),
    derive( Clone, Copy, Debug, PartialEq, Display, Into),
    derive_unsafe(whimsi_macros::WrapperToValue),
)]
pub struct DiskId(i16);
