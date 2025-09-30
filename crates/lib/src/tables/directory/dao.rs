use std::path::PathBuf;
use std::str::FromStr;

use derive_more::Constructor;
use getset::Getters;

use super::directory_identifier::DirectoryIdentifier;
use crate::tables::builder_list_entry::MsiBuilderListEntry;
use crate::tables::dao::IsDao;
use crate::types::column::default_dir::DefaultDir;
use crate::types::column::filename::Filename;
use crate::types::column::identifier::{Identifier, ToIdentifier};
use crate::types::helpers::to_msi_value::ToMsiOptionalValue;
use crate::types::helpers::to_unique_msi_identifier::ToUniqueMsiIdentifier;
use crate::types::properties::system_folder::SystemFolder;

#[derive(Clone, Debug, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct DirectoryDao {
    default_dir: DefaultDir,
    directory: DirectoryIdentifier,
    parent: Option<DirectoryIdentifier>,
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
            parent: Some(parent_id.into()),
        }
    }
}

impl IsDao for DirectoryDao {
    fn to_row(&self) -> Vec<whimsi_msi::Value> {
        vec![
            self.directory.clone().into(),
            self.parent.to_optional_value(),
            self.default_dir.clone().into(),
        ]
    }
}
impl MsiBuilderListEntry for DirectoryDao {
    // NOTE: We purposefully allow entries that have the same DefaultDir and
    // are contained by the same parent because you can assign
    // different components to these entries if you want
    // both components to install to the same location but based on separate
    // criteria.
    fn conflicts(&self, other: &Self) -> bool {
        self.directory == other.directory
    }
}

impl From<SystemFolder> for DirectoryDao {
    fn from(value: SystemFolder) -> Self {
        if value == SystemFolder::TARGETDIR {
            // Documentation says that only the root directory can have the same
            // ID for `parent` and `directory` fields.
            return Self {
                directory: value.into(),
                parent: None,
                default_dir: "SourceDir".parse::<Identifier>().unwrap().into(),
            };
        }
        Self {
            directory: value.into(),
            parent: Some(SystemFolder::TARGETDIR.into()),
            default_dir: Filename::parse(".").unwrap().into(),
        }
    }
}

impl ToUniqueMsiIdentifier for DirectoryDao {
    fn to_unique_msi_identifier(&self) -> Option<Identifier> {
        Some(self.directory.to_identifier())
    }
}

#[cfg(test)]
mod test {

    use crate::tables::dao::IsDao;
    use crate::tables::directory::dao::DirectoryDao;
    use crate::types::column::filename::Filename;
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
            whimsi_msi::Value::from("ProgramFilesFolder"),
            "Directory DAO directory name mismatch"
        );
        assert_eq!(
            *row.get(1).unwrap(),
            whimsi_msi::Value::from("TARGETDIR"),
            "Directory DAO parent name mismatch"
        );
        assert_eq!(
            *row.get(2).unwrap(),
            whimsi_msi::Value::from("."),
            "Directory DAO default dir name mismatch"
        );
    }
}
