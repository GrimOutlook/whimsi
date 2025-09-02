use uuid::Uuid;

#[derive(Clone, derive_more::Display, Debug, PartialEq)]
pub struct Guid(String);
impl From<Uuid> for Guid {
    fn from(value: Uuid) -> Self {
        todo!()
    }
}
