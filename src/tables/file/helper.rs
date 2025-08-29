use std::path::PathBuf;

use anyhow::{Context, ensure};
use getset::Getters;

use crate::{
    tables::component::helper::Component,
    types::{column::identifier::Identifier, helpers::filename::Filename},
};

#[derive(Clone, Debug, derive_more::Display, PartialEq, Getters)]
#[getset(get = "pub")]
#[display("{}", name)]
pub struct File {
    name: Filename,
    size: u64,
    component: Component,
}

impl TryFrom<PathBuf> for File {
    type Error = anyhow::Error;
    fn try_from(value: PathBuf) -> anyhow::Result<Self> {
        let path: PathBuf = value.into();
        ensure!(
            path.is_file(),
            FileConversionError::NotAFile { path: path.clone() }
        );

        let name = path
            .file_name()
            .ok_or(FileConversionError::NoFileName { path: path.clone() })?
            .to_str()
            .ok_or(FileConversionError::InvalidFileName { path: path.clone() })?
            .parse()?;

        // Should be able to just unwrap, since this has already been checked to be a valid file
        // and valid files must reside in a directory of some kind.
        let parent_directory = path.parent().unwrap().to_path_buf();

        let metadata = path.metadata()?;

        let size: u64;
        #[cfg(target_os = "linux")]
        {
            use std::os::unix::fs::MetadataExt;
            size = metadata.size();
        }
        #[cfg(target_os = "windows")]
        {
            use std::os::windows::fs::MetadataExt;
            size = metadata.file_size();
        }
        #[cfg(not(any(target_os = "linux", target_os = "windows")))]
        {
            compile_error!("Only Linux and Windows are supported currently.")
        }

        let component = Component::default();

        Ok(Self {
            name,
            size,
            component,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum FileConversionError {
    #[error("Path [{path}] is not a file")]
    NotAFile { path: PathBuf },
    #[error("Filename [{name}] already exists in parent directory")]
    DuplicateFile { name: String },
    #[error("No filename could be found for path [{path}]")]
    NoFileName { path: PathBuf },
    #[error("Invalid filename found for path [{path}]")]
    InvalidFileName { path: PathBuf },
}
