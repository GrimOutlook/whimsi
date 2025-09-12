use anyhow::{Context, Result};
use camino::Utf8PathBuf;
use flexstr::LocalStr;
use getset::Getters;
use uuid::Uuid;

use crate::traits::identifier::Identifier;

/// # [File](https://learn.microsoft.com/en-us/windows/win32/msi/file-table)
///
/// Represents a file that is to be copied from the MSI to the target system.
///
/// ## Properties
///
/// - `component_id` Internal identifier of the component that controls this
///   file. Must correspond to a tracked component_id.
/// - `file_id` Internal identifier of the file for the MSI. This must be
///   unique. Must correspond to a tracked file_id.
/// - `name` Filename of the file when placed on the system.
/// - `source` Path to the file when generating the MSI. Must correspond to a
///   file present on the system during MSI generation.
/// - `vital` Whether the entire install should fail if this file fails to be
///   installed.
/// - `size` The size of the file in bytes. This must be a non-negative number.
/// - `version` This field is the version string for a versioned file. This
///   field is blank for non-versioned files.
/// - `language` A list of decimal language IDs separated by commas.
#[derive(Clone, Debug, Getters)]
#[getset(get = "pub")]
pub(crate) struct MsiFile {
    component_id: LocalStr,
    file_id: LocalStr,
    source: Utf8PathBuf,
    name: LocalStr,
    size: u64,
    vital: bool,
    version: Option<String>,
    language: Option<String>,
}

impl MsiFile {
    pub fn new(source: &Utf8PathBuf) -> Result<MsiFile> {
        let metadata = source
            .metadata()
            .context(format!("Get metadata for {source}"))?;

        let size: u64;
        #[cfg(target_os = "linux")]
        {
            use std::os::unix::fs::MetadataExt;
            size = metadata.size();
        }
        #[cfg(target_os = "windows")]
        {
            use std::os::windows::fs::MetadataExt;
            let size = metadata.file_size();
        }
        #[cfg(not(any(target_os = "linux", target_os = "windows")))]
        {
            compile_error!("Only Linux and Windows are supported currently.")
        }

        let file = MsiFile {
            component_id: Uuid::as_identifier(),
            file_id: Uuid::as_identifier(),
            source: source.into(),
            name: source.to_string().into(),
            size,
            vital: false,
            version: None,
            language: None,
        };

        Ok(file)
    }
}
