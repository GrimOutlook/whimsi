use flexstr::LocalStr;
use serde::Deserialize;
use serde_inline_default::serde_inline_default;

/// # Product Information Properties
///
/// All properties in this section are required for all MSI installations.
///
/// [Reference](https://learn.microsoft.com/en-us/windows/win32/msi/property-reference)
#[serde_inline_default] // This must come before deriving serde modules.
#[derive(Deserialize)]
#[serde(rename = "product_info")]
pub(crate) struct ProductConfig {
    /// The name of the application to be installed.
    ///
    /// [Reference](https://learn.microsoft.com/en-us/windows/win32/msi/productname)
    pub(crate) name: LocalStr,
    /// The version of the application to be installed. The format is
    /// \[MAJOR].\[MINOR].\[BUILD]
    ///
    /// [Reference](https://learn.microsoft.com/en-us/windows/win32/msi/productversion)
    pub(crate) version: LocalStr,
    /// The name of the manufacturer for the application that is being installed.
    ///
    /// [Reference](https://learn.microsoft.com/en-us/windows/win32/msi/manufacturer)
    pub(crate) manufacturer: LocalStr,
    /// Specifies the language the installer should use for any strings in the
    /// user interface that are not authored into the database. This property must
    /// be a numeric language identifier.
    ///   - TODO: Figure out where to find the official definitions of these for
    ///     users. I believe English is 1033.
    ///
    /// [Reference](https://learn.microsoft.com/en-us/windows/win32/msi/productlanguage)
    #[serde_inline_default(1033)]
    pub(crate) language: u16,
    /// A unique identifier for the particular product release, represented as a
    /// string GUID. This ID must vary for different versions and languages. If not specified the
    /// application will generate a random GUID.
    ///
    /// [Reference](https://learn.microsoft.com/en-us/windows/win32/msi/productcode)
    pub(crate) product_code: Option<LocalStr>,
}
