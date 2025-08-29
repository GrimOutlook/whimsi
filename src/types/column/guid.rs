use uuid::Uuid;

#[derive(Clone, Debug, PartialEq)]
pub struct Guid(String);
impl From<Uuid> for Guid {
    fn from(value: Uuid) -> Self {
        todo!()
    }
}
