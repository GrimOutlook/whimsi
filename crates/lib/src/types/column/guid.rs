use uuid::Uuid;

use crate::types::helpers::to_msi_value::IntoMsiValue;

#[derive(
    Clone, Debug, PartialEq, derive_more::Display, whimsi_macros::StrToValue,
)]
pub struct Guid(String);
impl From<Uuid> for Guid {
    fn from(value: Uuid) -> Self {
        todo!()
    }
}
