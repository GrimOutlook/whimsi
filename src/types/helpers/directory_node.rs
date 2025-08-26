use std::{cell::RefCell, rc::Rc};

use super::directory::Directory;

/// Represents items that can be contained by a directory
#[derive(Debug, Clone, PartialEq, strum::EnumIs, strum::EnumTryAs, derive_more::From)]
pub enum DirectoryItem {
    // File(File),
    Directory(Directory),
    // Shortcut(Shortcut),
}
