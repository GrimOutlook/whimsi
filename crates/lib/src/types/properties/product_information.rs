pub enum ProductInformationProperty {
    /// Internet address or URL for technical support.
    ARPHELPLINK,
    /// Technical support phone numbers.
    ARPHELPTELEPHONE,
    /// String displayed by a message box that prompts for a disk.
    DiskPrompt,
    /// Set to 1 (one) if the current installation is running from a package
    /// created through an administrative installation.
    IsAdminPackage,
    /// Places units to the left of the number.
    LeftUnit,
    /// Name of the application manufacturer. (Required)
    Manufacturer,
    /// The installer sets this property to 1 (one) when the installation uses
    /// a media source, such as a CD-ROM.
    MediaSourceDir,
    /// The presence of this property indicates that a product code changing
    /// transform is registered to the product.
    MSIINSTANCEGUID,
    /// This property indicates the installation of a new instance of a product
    /// with instance transforms.
    MSINEWINSTANCE,
    /// The installer sets this property for installations that a Concurrent
    /// Installation action runs.
    ParentProductCode,
    /// String used as a template for the PIDKEY property.
    PIDTemplate,
    /// A unique identifier for a specific product release. (Required)
    ProductCode,
    /// Numeric language identifier (LANGID) for the database. (REQUIRED)
    ProductLanguage,
    /// Human readable name of an application. (Required)
    ProductName,
    /// Set to the installed state of a product.
    ProductState,
    /// String format of the product version as a numeric value. (Required)
    ProductVersion,
    /// A GUID that represents a related set of products.
    UpgradeCode,
}
