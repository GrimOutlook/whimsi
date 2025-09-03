use super::identifier::Identifier;

#[derive(Clone, Debug, PartialEq)]
pub struct Sequence {
    inner: i16,
    media: Identifier,
}

impl Into<i16> for &Sequence {
    fn into(self) -> i16 {
        self.inner
    }
}
