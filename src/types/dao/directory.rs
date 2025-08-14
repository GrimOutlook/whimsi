use std::path::PathBuf;

use derive_more::Constructor;
use getset::Getters;

use crate::types::{
    column::{default_dir::DefaultDir, identifier::Identifier},
    helpers::filename::Filename,
    properties::system_folder::SystemFolder,
};

#[derive(Clone, Debug, PartialEq, Getters, Constructor)]
#[getset(get = "pub")]
pub struct DirectoryDao {
    default_dir: DefaultDir,
    directory: Identifier,
    parent: Identifier,
}

impl From<SystemFolder> for DirectoryDao {
    fn from(value: SystemFolder) -> Self {
        Self {
            directory: value.into(),
            parent: SystemFolder::TARGETDIR.into(),
            default_dir: Filename::parse_with_trim(".").unwrap().into(),
        }
    }
}

impl TryFrom<PathBuf> for DirectoryDao {
    type Error = anyhow::Error;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        todo!()
    }
}

#[cfg(test)]
mod test {

    use crate::types::{
        dao::directory::DirectoryDao,
        helpers::filename::Filename,
        properties::system_folder::SystemFolder::{self},
    };

    #[test]
    fn try_from() {
        let pf_dao: DirectoryDao = SystemFolder::ProgramFiles.into();
        assert_eq!(
            *pf_dao.default_dir(),
            Filename::parse(".")
                .expect("Failed to parse `.` directory name to Identifier for system folder.")
                .into()
        )
    }
}
