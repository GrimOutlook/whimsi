use std::path::PathBuf;

use anyhow::Context;
use anyhow::bail;
use derive_more::Constructor;
use getset::Getters;
use msi::Language;

use crate::dint_val;
use crate::int_val;
use crate::opt_str_val;
use crate::opt_val;
use crate::str_val;
use crate::tables::dao::IsDao;
use crate::types::column::attributes::Attributes;
use crate::types::column::filename::Filename;
use crate::types::column::identifier::Identifier;
use crate::types::column::sequence::Sequence;
use crate::types::column::version::Version;

#[derive(Clone, Debug, Default, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct FileDao {
    file: Identifier,
    component: Identifier,
    name: Filename,
    size: i32,
    version: Option<Version>,
    language: Option<Language>,
    attributes: Option<i16>,
    sequence: Sequence,
}

impl FileDao {
    /// Create a FileDao for a file that is to be installed when the MSI is run.
    pub(crate) fn install_file(
        file_id: Identifier,
        component_id: Identifier,
        name: Filename,
        size: i32,
        sequence: Sequence,
    ) -> anyhow::Result<FileDao> {
        let Sequence::Included(_) = sequence else {
            // TODO: Create real error
            bail!("Sequence number cannot be 0 for included files")
        };
        Ok(Self {
            file: file_id,
            component: component_id,
            name,
            size,
            version: None,
            language: None,
            attributes: None,
            sequence,
        })
    }

    /// Create FileDao for a file at a given path, that is to be installed when
    /// the MSI is run.
    pub(crate) fn install_file_from_path(
        file_id: Identifier,
        component_id: Identifier,
        path: PathBuf,
        sequence: Sequence,
    ) -> anyhow::Result<FileDao> {
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

        if size > i32::MAX.try_into().unwrap() {
            bail!(
                "[{:?}]'s size [{} bytes] is too large to be stored in the MSI database. Largest size in bytes allowed is {}.",
                path,
                size,
                i32::MAX
            )
        }

        let size = i32::try_from(size).unwrap();

        let name = path
            .file_name()
            .with_context(|| {
                format!(
                    "Path [{path:?}] terminates in `..` which is not allowed."
                )
            })?
            .to_str()
            .with_context(|| format!("Path [{path:?}] is not valid unicode."))?
            .parse()
            .with_context(|| {
                format!(
                    "Path [{path:?}] terminates in `..` which is not allowed"
                )
            })?;

        Self::install_file(file_id, component_id, name, size, sequence)
    }
}

impl IsDao for FileDao {
    fn to_row(&self) -> Vec<msi::Value> {
        vec![
            str_val!(self.file),
            str_val!(self.component),
            str_val!(self.name),
            dint_val!(self.size),
            opt_str_val!(self.version),
            opt_val!(self.language),
            opt_val!(self.attributes),
            int_val!(Into::<i16>::into(self.sequence.clone())),
        ]
    }

    fn conflicts(&self, other: &Self) -> bool {
        self.file == other.file
    }
}
