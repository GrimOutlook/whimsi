use std::{path::PathBuf, str::FromStr};

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

impl DirectoryDao {
    pub(crate) fn from_path(
        path: PathBuf,
        path_id: Identifier,
        parent_id: Identifier,
    ) -> anyhow::Result<Self> {
        // Will panic if path terminates in '..' or basename is not valid Unicode.
        let name = path.file_name().unwrap().to_str().unwrap();
        Ok(Self {
            default_dir: DefaultDir::Filename(Filename::parse(name)?),
            directory: path_id,
            parent: parent_id,
        })
    }
}

impl From<SystemFolder> for DirectoryDao {
    fn from(value: SystemFolder) -> Self {
        if value == SystemFolder::TARGETDIR {
            // Documentation says that only the root directory can have the same ID for `parent`
            // and `directory` fields.
            return Self {
                directory: value.into(),
                parent: value.into(),
                default_dir: "SourceDir".parse::<Identifier>().unwrap().into(),
            };
        }
        Self {
            directory: value.into(),
            parent: SystemFolder::TARGETDIR.into(),
            default_dir: Filename::parse(".").unwrap().into(),
        }
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
            Filename::strict_parse(".")
                .expect("Failed to parse `.` directory name to Identifier for system folder.")
                .into()
        )
    }
}
