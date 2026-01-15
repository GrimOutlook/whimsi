use crate::types::helpers::to_msi_value::IntoMsiValue;

/// Microsoft sure loves their weird standards. We're bit packing date times but
/// in every MSI they store the default error message.
///
/// [*Reference*](https://learn.microsoft.com/en-us/windows/win32/msi/signature-table#remarks)
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Date {
    date: u16,
    time: u16,
}

impl IntoMsiValue for Date {
    fn into(&self) -> msi::Value {
        todo!()
    }
}
