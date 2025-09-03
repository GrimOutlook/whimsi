use pastey::paste;
use std::path::PathBuf;

use anyhow::{Context, bail, ensure};
use derivative::Derivative;
use getset::Getters;

use crate::{
    tables::component::helper::Component,
    types::{column::identifier::Identifier, helpers::filename::Filename},
};

use super::attributes::FileAttributes;

/// Most file information is gathered during the user input period but a few things need to be
/// generated during the convertions to the `msi` crate's `Package` type. A `ComponentTable`
/// listing must be generated for each `File` object. Based on the information present in the
/// component a `Feature` listing must be generated as well. The `Media` information is also
/// generated during the conversion so multiple files can be placed on the same `MediaTable`
/// listing. The file is also placed into a `.cab` stream so it can be loaded into the `Package`
/// and referenced by the `MediaTable` listing
#[derive(Clone, Debug, derive_more::Display, PartialEq, Getters, Derivative)]
#[getset(get = "pub")]
#[display("{}", name)]
#[derivative(PartialOrd, Ord, Eq)]
pub struct File {
    name: Filename,
    #[derivative(PartialOrd = "ignore", Ord = "ignore")]
    full_path: PathBuf,
    #[derivative(PartialOrd = "ignore", Ord = "ignore")]
    size: i32,
    #[derivative(PartialOrd = "ignore", Ord = "ignore")]
    component: Component,
    #[derivative(PartialOrd = "ignore", Ord = "ignore")]
    attributes: i16,

    /// The source media that this file should be added to.
    ///
    /// If one is not provided, one will be created automatically.
    #[derivative(PartialOrd = "ignore", Ord = "ignore")]
    media: Option<Identifier>,
}

macro_rules! gen_attr_methods {
    ($function_name_stub:ident) => {
        paste! {
            pub fn [<set_ $function_name_stub>](&mut self, value: bool) {
                if value {
                    self.attributes |= FileAttributes::[<$function_name_stub:camel>].bits();
                } else {
                    self.attributes &= FileAttributes::[<$function_name_stub:camel>].not().bits();
                }
            }

            pub fn [<is_ $function_name_stub>](&self) -> bool {
                ( FileAttributes::[<$function_name_stub:camel>].bits() & self.attributes ) > 0
            }
        }
    };
}

impl File {
    gen_attr_methods!(read_only);
    gen_attr_methods!(hidden);
    gen_attr_methods!(system);
    gen_attr_methods!(vital);
    // TODO:
    // Do compressed and non-compressed. These cannot be set at the same time. So they cannot use
    // the default macro.
}

impl TryFrom<PathBuf> for File {
    type Error = anyhow::Error;
    fn try_from(value: PathBuf) -> anyhow::Result<Self> {
        let path: PathBuf = value.clone().into();
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
        let Ok(size) = i32::try_from(size) else {
            bail!(FileConversionError::FileTooLarge { path, size });
        };

        Ok(Self {
            full_path: value,
            name,
            size,
            attributes: (0 as i16).into(),
            component,
            media: None,
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
    #[error("File is too large for MSI Files Table. Size must be less than 2GB.")]
    FileTooLarge { path: PathBuf, size: u64 },
}

#[cfg(test)]
mod test {
    use assert_fs::{
        NamedTempFile, TempDir,
        fixture::ChildPath,
        prelude::{FileTouch, PathChild},
    };

    use super::File;

    fn create_test_file() -> NamedTempFile {
        let file_1 = NamedTempFile::new("file1.txt").unwrap();
        file_1.touch().unwrap();

        file_1
    }

    #[test]
    fn set_vital() {
        let test_file = create_test_file();
        let mut file = File::try_from(test_file.to_path_buf()).unwrap();
        assert!(!file.is_vital());
        file.set_vital(true);
        assert!(file.is_vital());
    }
}
