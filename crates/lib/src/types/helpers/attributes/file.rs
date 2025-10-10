use bitflags::bitflags;

use crate::types::column::integer::Integer;
bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, whimsi_macros::BitmaskToValue)]
    pub struct FileAttributes: Integer {
    }
}
