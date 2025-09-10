#[derive(Clone, Debug, PartialEq)]
pub struct Condition;

impl ToString for Condition {
    fn to_string(&self) -> String {
        todo!()
    }
}

impl From<Condition> for msi::Value {
    fn from(value: Condition) -> Self {
        value.to_string().into()
    }
}
