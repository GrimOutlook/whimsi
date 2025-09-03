use derivative::Derivative;
use getset::Getters;

use crate::{
    implement_directory_kind_boilerplate,
    types::{helpers::directory_item::DirectoryItem, properties::system_folder::SystemFolder},
};

use super::kind::DirectoryKind;

#[derive(Clone, Debug, Derivative, Getters, PartialEq, derive_more::Display)]
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

    fn name_conflict(&self, other: &Self) -> bool {
        self.system_folder == other.system_folder
    }
}

impl From<SystemFolder> for SystemDirectory {
    fn from(value: SystemFolder) -> Self {
        Self {
            contained: Vec::new(),
            system_folder: value,
        }
    }
}
