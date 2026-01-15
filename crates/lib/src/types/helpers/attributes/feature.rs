use bitflags::bitflags;

use crate::types::column::integer::Integer;
use crate::types::helpers::to_msi_value::IntoMsiValue;
bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, whimsi_macros::BitmaskToValue)]
    pub struct FeatureAttributes: Integer {
    }
}
