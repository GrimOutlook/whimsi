use std::str::FromStr;

use derivative::Derivative;
use getset::{Getters, Setters, WithSetters};
use whimsi_msi::{Language, PackageType};

use crate::types::{
    column::version::Version,
    helpers::{architecture::MsiArchitecture, security_flag::DocSecurity},
};

#[derive(Debug, Clone, Getters, Setters, WithSetters)]
#[getset(get = "pub", set = "pub", set_with = "pub")]
pub struct MetaInformation {
    package_type: PackageType,
    subject: String,
    // version: Version,
    author: Option<String>,
    manufacturer: Option<String>,
    architecture: Option<MsiArchitecture>,
    languages: Vec<Language>,
    comments: Option<String>,
    keywords: Vec<String>,
    /// Leaving this blank will cause the build to default it to `ReadOnlyRecommended` when the
    /// `PackageType` is `Installer` and `ReadOnlyEnforced` for `PackageType` `Transform` and
    /// `Patch`.
    security: Option<DocSecurity>,
}

impl MetaInformation {
    pub fn new(package_type: PackageType, subject: String) -> Self {
        MetaInformation {
            package_type,
            subject,
            author: None,
            manufacturer: None,
            architecture: None,
            comments: None,
            keywords: Vec::new(),
            languages: Vec::new(),
            security: None,
        }
    }
}
