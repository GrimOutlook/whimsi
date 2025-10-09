/// The this Language data type is distinct from the Language datatype in the msi crate because the
/// `msi` crate `Language` datatype represents a single language, while this datatype represents a
/// column containing language data which can consist of multiple language codes.
///
/// [*Reference*](https://learn.microsoft.com/en-us/windows/win32/msi/language)
#[derive(Clone, Debug, PartialEq)]
pub struct Language(Vec<msi::Language>);

impl msi::ToValue for Language {
    fn to_value(&self) -> msi::Value {
        todo!()
    }
}
