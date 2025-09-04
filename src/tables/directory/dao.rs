use std::path::PathBuf;
use std::str::FromStr;

use derive_more::Constructor;
use getset::Getters;

use super::directory_identifier::DirectoryIdentifier;
use crate::str_val;
use crate::types::column::default_dir::DefaultDir;
use crate::types::column::identifier::Identifier;
use crate::types::helpers::filename::Filename;
use crate::types::properties::system_folder::SystemFolder;

#[derive(Clone, Debug, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct DirectoryDao {
    default_dir: DefaultDir,
    directory: DirectoryIdentifier,
    parent: DirectoryIdentifier,
}

impl DirectoryDao {
    pub(crate) fn new(
        name: impl Into<DefaultDir>,
        path_id: impl Into<DirectoryIdentifier>,
        parent_id: impl Into<DirectoryIdentifier>,
    ) -> Self {
        Self {
            default_dir: name.into(),
            directory: path_id.into(),
            parent: parent_id.into(),
        }
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
        if value == SystemFolder::TARGETDIR {
            // Documentation says that only the root directory can have the same
            // ID for `parent` and `directory` fields.
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
    use crate::types::helpers::filename::Filename;
    use crate::types::properties::system_folder::SystemFolder;

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
