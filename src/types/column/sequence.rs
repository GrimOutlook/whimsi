use super::identifier::Identifier;

#[derive(Clone, Debug, PartialEq)]
pub enum Sequence {
    Included(IncludedSequence),
    NotIncluded,
}

#[derive(Clone, Debug, PartialEq)]
pub struct IncludedSequence {
    inner: i16,
    media: Identifier,
}

impl Into<i16> for IncludedSequence {
    fn into(self) -> i16 {
        self.inner
    }
}

impl Into<i16> for Sequence {
    fn into(self) -> i16 {
        match self {
            Sequence::Included(included_sequence) => included_sequence.into(),
            Sequence::NotIncluded => 0,
        }
    }
}
