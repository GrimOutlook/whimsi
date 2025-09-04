use std::path::PathBuf;

pub(crate) mod dao;
pub mod helper;
pub(crate) mod kind;
pub mod system_directory;
pub mod table;
pub mod traits;

#[derive(Debug, thiserror::Error)]
pub enum DirectoryError {
    #[error("Given directory name [{name}] cannot fit in short filename")]
    DirectoryNameTooLong { name: String },
    #[error("Directory name [{name}] already exists in parent directory")]
    DuplicateDirectory { name: String },
    #[error("No directory name could be found for path [{path}]")]
    NoDirectoryName { path: PathBuf },
    #[error("Invalid directory name found for path [{path}]")]
    InvalidDirectoryName { path: PathBuf },
    #[error("File [{name}] already exists in directory")]
    DuplicateFile { name: String },
}
