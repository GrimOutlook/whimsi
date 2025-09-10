use flexstr::LocalStr;
use serde::Deserialize;
use serde_inline_default::serde_inline_default;

/// # Summary Information Properties
///
/// [Reference](https://learn.microsoft.com/en-us/windows/win32/msi/summary-property-descriptions)
#[serde_inline_default]
#[derive(Deserialize)]
pub(crate) struct SummaryConfig {
    /// *Required*
    ///
    /// Contains the minimum installer version required by the installation
    /// package.
    ///
    /// [Reference](https://learn.microsoft.com/en-us/windows/win32/msi/page-count-summary)
    #[serde_inline_default(200)]
    pub(crate) page_count: u16,
    /// Optional in config, required by MSI.
    ///
    /// Contains the package code (GUID) for the installer package. If one is not provided, one
    /// will be generated at random.
    ///   - TODO: How does this relate to the product_code GUID?
    ///   - TODO: Can this be automatically generated? If it can add a note to
    ///     this comment section saying so.
    ///
    /// [Reference](https://learn.microsoft.com/en-us/windows/win32/msi/revision-number-summary)
    pub(crate) revision_number: Option<LocalStr>,
    /// Optional in config, required by MSI. If one is not provided one will be generated from the
    /// provided required information.
    ///
    /// The platform and languages compatible with this installation package.
    ///
    /// [Resource](https://learn.microsoft.com/en-us/windows/win32/msi/template-summary)
    pub(crate) template: Option<LocalStr>,
    /// Optional in config, required by MSI.
    ///
    /// The type of the source file image.
    ///
    /// [Reference](https://learn.microsoft.com/en-us/windows/win32/msi/word-count-summary)
    pub(crate) word_count: Option<u16>,
    /// The name of the author publishing the installation package, transform, or
    /// patch package.
    ///
    /// [Reference](https://learn.microsoft.com/en-us/windows/win32/msi/author-summary)
    pub(crate) author: Option<String>,
    /// The numeric value of the ANSI code page used for any strings that are
    /// stored in the summary information
    ///
    /// [Reference](https://learn.microsoft.com/en-us/windows/win32/msi/codepage-summary)
    #[serde_inline_default(Some(lcid::constants::LANG_EN_US.ansi_code_page.unwrap().into()))]
    pub(crate) code_page: Option<u32>,
    /// Conveys the general purpose of the installation package, transform, or
    /// patch package.
    ///
    /// [Reference](https://learn.microsoft.com/en-us/windows/win32/msi/comments-summary)
    pub(crate) comments: Option<String>,
    /// Contains the name of the software used to author this MSI. If this is not
    /// set in the config, it is populated with "whimsi v[WHIMSI_VERSION]".
    ///
    /// [Reference](https://learn.microsoft.com/en-us/windows/win32/msi/creating-application-summary)
    pub(crate) generating_application: Option<String>,

    /// A list of keywords that may be used by file browsers to do keyword searches for a file.
    /// Based on the MSI type the following strings will be appended to this list even if no
    /// keywords are supplied:
    /// - `Installer`: 'Installer'
    /// - `Patch`: 'Patch'
    /// - `Transform`: 'Transform'
    pub(crate) keywords: Option<Vec<String>>,
}
