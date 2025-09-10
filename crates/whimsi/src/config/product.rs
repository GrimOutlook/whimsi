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
    /// TODO: Change this to accept `lcdi::LanguageId` objects. Have to implement deserialize
    /// manually since the crate doesn't have it natively. Just check if the config value is a number or a
    /// string and match on either the `lcid` property or the `name` property respectively.
    ///
    /// Specifies the language the installer should use for any strings in the
    /// user interface that are not authored into the database. Any value that can be parsed by
    /// the `lcid` crate can be used. According to the crate documentation this includes `u32`
    /// Integers (such as 1033 for American English) or a string (such as en_US).
    ///
    /// The language IDs can be found on [this
    /// page](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-lcid/70feba9f-294e-491e-b6eb-56532684c37f).
    /// NOTE: The language IDs given by the user are parsed by the `lcid` crate which is still
    /// using the 15.0 protocol revision.
    ///
    /// [Reference](https://learn.microsoft.com/en-us/windows/win32/msi/productlanguage)
    #[serde_inline_default(lcid::constants::LANG_EN_US.lcid)]
    pub(crate) language: u32,
    /// A unique identifier for the particular product release, represented as a
    /// string GUID. This ID must vary for different versions and languages. If not specified the
    /// application will generate a random GUID.
    ///
    /// [Reference](https://learn.microsoft.com/en-us/windows/win32/msi/productcode)
    pub(crate) product_code: Option<LocalStr>,
}
