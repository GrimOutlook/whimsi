use std::{cell::RefCell, fs::DirEntry, path::PathBuf, rc::Rc};

use anyhow::bail;
use camino::Utf8PathBuf;
use itertools::Itertools;

use crate::tables::{directory::helper::Directory, file::helper::File};

/// Represents items that can be contained by a directory
#[derive(Debug, Clone, PartialEq, strum::EnumIs, strum::EnumTryAs, derive_more::From)]
pub enum DirectoryItem {
    File(File),
    Directory(Directory),
    // Shortcut(Shortcut),
}

impl TryFrom<PathBuf> for DirectoryItem {
    type Error = anyhow::Error;

    /// Parses throught a path recursively and returns the contents found.
    ///
    /// If the `PathBuf` points to a directory, a `Directory` object of variant `SubDirectory` will
    /// be returned where the `contents` attribute of the object will contain the contents of the
    /// directory in the filesystem
    /// Contained directories are recursively read and included.
    /// If the `PathBuf` points to a file, only the file is returned.
    fn try_from(path: PathBuf) -> anyhow::Result<DirectoryItem> {
        let result = if path.is_dir() {
            let directory: Directory = path.try_into()?;
            directory.into()
        } else if path.is_file() {
            let file: File = path.try_into()?;
            file.into()
        } else {
            todo!("Make error for not-file+not-directory this case")
        };

        Ok(result)
    }
}
