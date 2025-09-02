use derivative::Derivative;
use getset::Getters;

use crate::{
    implement_directory_kind_boilerplate,
    types::{helpers::directory_item::DirectoryItem, properties::system_folder::SystemFolder},
};

use super::kind::DirectoryKind;

#[derive(Clone, Debug, derive_more::Display, PartialEq, Getters, Derivative)]
#[display("{}", system_folder)]
#[getset(get = "pub")]
#[derivative(PartialOrd, Ord, Eq)]
pub struct SystemDirectory {
    #[getset(skip)]
    #[derivative(PartialOrd = "ignore", Ord = "ignore")]
    contained: Vec<DirectoryItem>,
    system_folder: SystemFolder,
}

impl DirectoryKind for SystemDirectory {
    implement_directory_kind_boilerplate!();
}

impl From<SystemFolder> for SystemDirectory {
    fn from(value: SystemFolder) -> Self {
        Self {
            contained: Vec::new(),
            system_folder: value,
        }
    }
}
