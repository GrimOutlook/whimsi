use bitflags::bitflags;

use crate::types::column::integer::Integer;
bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct ComponentAttributes: Integer {
    }
}

impl msi::ToValue for ComponentAttributes {
    fn to_value(&self) -> msi::Value {
        msi::Value::Int(self.bits().into())
    }
}
