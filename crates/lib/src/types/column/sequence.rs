use super::identifier::Identifier;

#[derive(
    Clone, Debug, Default, PartialEq, strum::EnumTryAs, derive_more::Display,
)]
#[display("{}", Into::<i32>::into(self.clone()))]
pub enum Sequence {
    Included(IncludedSequence),
    // I was curious if there was a usecase where a FileTable entry could be
    // made for a file that you didn't want to copy from the MSI media
    // such as duplicating an already existing file and Gemini said that
    // in those cases a value of 0 is used by tools like WiX and Advanced
    // Installer. The Microsoft documentation explicitly states that the value
    // of the sequence number field must be "greater than or equal to one"
    // and Gemini would not supply links to a reference like it sometimes
    // can. I'm taking it on it's word for now as I can't find any more
    // information about this case but this may result in errors if used.
    // Marking it as an issue for investigation.
    // TODO: Verify this doesn't cause issues.
    #[default]
    NotIncluded,
}

#[derive(Clone, Debug, PartialEq, derive_more::Constructor)]
pub struct IncludedSequence {
    inner: i32,
}

impl IncludedSequence {
    pub fn to_i16(&self) -> i32 {
        self.inner
    }
}

impl Into<i32> for IncludedSequence {
    fn into(self) -> i32 {
        self.inner
    }
}

impl Into<i32> for Sequence {
    fn into(self) -> i32 {
        match self {
            Sequence::Included(included_sequence) => included_sequence.into(),
            Sequence::NotIncluded => 0,
        }
    }
}

impl From<Sequence> for whimsi_msi::Value {
    fn from(value: Sequence) -> Self {
        Into::<i32>::into(value).into()
    }
}
