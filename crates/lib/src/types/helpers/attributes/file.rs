use bitflags::bitflags;

use crate::types::column::integer::Integer;
bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct FileAttributes: Integer {
    }
}

impl msi::ToValue for FileAttributes {
    fn to_value(&self) -> msi::Value {
        msi::Value::Int(self.bits().into())
    }
}
