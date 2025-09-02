use derivative::Derivative;
use getset::Getters;
use msi::PackageType;
use std::str::FromStr;

use crate::types::column::version::Version;

#[derive(Debug, Clone, Derivative, Getters)]
#[derivative(Default)]
#[get = "pub"]
pub struct MetaInformation {
    #[derivative(Default(value = "PackageType::Installer"))]
    package_type: PackageType,
    package_name: String,
    #[derivative(Default(value = "Version::from_str(\"0\").unwrap()"))]
    version: Version,
}
