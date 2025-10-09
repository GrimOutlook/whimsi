#[derive(Clone, Debug, PartialEq)]
pub struct Condition;

impl ToString for Condition {
    fn to_string(&self) -> String {
        todo!()
    }
}

impl msi::ToValue for Condition {
    fn to_value(&self) -> msi::Value {
        self.to_string().into()
    }
}
