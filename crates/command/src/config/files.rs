use serde::Deserialize;

/// Supports setting
#[derive(Deserialize)]
#[serde(rename = "files")]
pub(crate) struct FilesConfig {
    pub(crate) program_files_x86: Option<String>,
    pub(crate) program_files: Option<String>,
    pub(crate) desktop: Option<String>,
}
