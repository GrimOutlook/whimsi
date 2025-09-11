use std::str::FromStr;

use derivative::Derivative;
use getset::{Getters, Setters, WithSetters};
use whimsi_msi::{Language, PackageType};

use crate::types::{
    column::version::Version, helpers::architecture::MsiArchitecture,
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
}

impl MetaInformation {
    pub fn new(package_type: PackageType, subject: String) -> Self {
        MetaInformation {
            package_type,
            subject,
            author: None,
            manufacturer: None,
            architecture: None,
            languages: Vec::new(),
        }
    }
}
