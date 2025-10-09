use bitflags::bitflags;

use crate::types::column::integer::Integer;
bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq)]
    struct FeatureAttributes: Integer {
    }
}

impl msi::ToValue for FeatureAttributes {
    fn to_value(&self) -> msi::Value {
        msi::Value::Int(self.bits().into())
    }
}
