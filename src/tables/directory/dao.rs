use std::{path::PathBuf, str::FromStr};

use derive_more::Constructor;
use getset::Getters;

use crate::{
    str_val,
    types::{
        column::{default_dir::DefaultDir, identifier::Identifier},
        helpers::filename::Filename,
        properties::system_folder::SystemFolder,
    },
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

    pub fn to_row(&self) -> Vec<msi::Value> {
        vec![
            str_val!(self.directory),
            str_val!(self.parent),
            str_val!(self.default_dir),
        ]
    }
}

impl From<SystemFolder> for DirectoryDao {
    fn from(value: SystemFolder) -> Self {
        DirectoryDao::from(&value)
    }
}

impl From<&SystemFolder> for DirectoryDao {
    fn from(value: &SystemFolder) -> Self {
        if value == &SystemFolder::TARGETDIR {
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

    use crate::tables::directory::dao::DirectoryDao;
    use crate::types::{helpers::filename::Filename, properties::system_folder::SystemFolder};

    #[test]
    fn try_from() {
        let pf_dao: DirectoryDao = SystemFolder::ProgramFilesFolder.into();
        assert_eq!(
            *pf_dao.default_dir(),
            Filename::strict_parse(".")
                .expect("Failed to parse `.` directory name to Identifier for system folder.")
                .into()
        )
    }

    #[test]
    fn to_row() {
        let pf_dao: DirectoryDao = SystemFolder::ProgramFilesFolder.into();
        let row = pf_dao.to_row();
        assert_eq!(row.len(), 3, "Directory DAO row number mismatch");
        assert_eq!(
            *row.get(0).unwrap(),
            msi::Value::from("ProgramFilesFolder"),
            "Directory DAO directory name mismatch"
        );
        assert_eq!(
            *row.get(1).unwrap(),
            msi::Value::from("TARGETDIR"),
            "Directory DAO parent name mismatch"
        );
        assert_eq!(
            *row.get(2).unwrap(),
            msi::Value::from("."),
            "Directory DAO default dir name mismatch"
        );
    }
}
