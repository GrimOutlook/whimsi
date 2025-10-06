/// This datatype is only available for Windows Installer 5.0 and later.
#[derive(Debug, Clone, Default, derive_more::Display)]
pub struct FormattedSddlText {
    inner: String,
}
