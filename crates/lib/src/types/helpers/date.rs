use msi::ToValue;

/// Microsoft sure loves their weird standards. We're bit packing date times but in every MSI they
/// store the default error message.
///
/// [*Reference*](https://learn.microsoft.com/en-us/windows/win32/msi/signature-table#remarks)
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Date {
    date: u16,
    time: u16,
}

impl ToValue for Date {
    fn to_value(&self) -> msi::Value {
        todo!()
    }
}
