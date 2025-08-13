use derive_more::Constructor;
use getset::Getters;

use crate::types::{
    column::{default_dir::DefaultDir, identifier::Identifier},
    helpers::directory::SystemDirectory,
    properties::system_folder::SystemFolder,
};

#[derive(Clone, Debug, PartialEq, Getters, Constructor)]
#[getset(get = "pub")]
pub(crate) struct DirectoryDao {
    default_dir: DefaultDir,
    directory: Identifier,
    parent: Identifier,
}

impl TryFrom<&SystemDirectory> for DirectoryDao {
    type Error = anyhow::Error;
    fn try_from(value: &SystemDirectory) -> Result<Self, Self::Error> {
        let dir = Self {
            directory: value.id().clone().into(),
            parent: SystemFolder::TARGETDIR.into(),
            default_dir: value.name().clone().into(),
        };
        Ok(dir)
    }
}

#[cfg(test)]
mod test {

    use crate::types::{
        dao::directory::DirectoryDao,
        helpers::{
            directory::{Directory, SystemDirectory},
            filename::Filename,
        },
        properties::system_folder::SystemFolder::ProgramFiles,
    };

    #[test]
    fn try_from() {
        let dir: SystemDirectory = Directory::system_directory(ProgramFiles);
        let pf_dao: DirectoryDao = (&dir)
            .try_into()
            .expect("Failed to convert program files system folder to DAO");
        assert_eq!(
            *pf_dao.default_dir(),
            Filename::parse(".")
                .expect("Failed to parse `.` directory name to Identifier for system folder.")
                .into()
        )
    }
}
