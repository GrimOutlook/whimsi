// TODO: Implement this.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Binary;

impl From<Binary> for whimsi_msi::Value {
    fn from(value: Binary) -> Self {
        // NOTE: The actual binary data needs to be written to it's own stream
        // in the MSI. Writing a blank string is seemingly what is supposed to
        // be done for binary represented columns.
        whimsi_msi::Value::Str("".to_string())
    }
}
