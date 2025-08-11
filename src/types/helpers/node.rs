use std::{cell::RefCell, rc::Rc};

use super::directory::SubDirectory;

/// Represents items that can be contained by a directory
#[derive(Debug, Clone, PartialEq, strum::EnumIs, strum::EnumTryAs, derive_more::From)]
pub enum Node {
    // File(File),
    Directory(Rc<RefCell<SubDirectory>>),
    // Shortcut(Shortcut),
}
