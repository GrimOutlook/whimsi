use uuid::Uuid;

#[derive(Clone, derive_more::Display, Debug, PartialEq)]
pub struct Guid(String);
impl From<Uuid> for Guid {
    fn from(value: Uuid) -> Self {
        todo!()
    }
}

impl From<Guid> for whimsi_msi::Value {
    fn from(value: Guid) -> Self {
        value.0.into()
    }
}
