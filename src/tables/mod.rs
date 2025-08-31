pub(crate) mod builder_table;
pub mod component;
pub mod directory;
pub mod file;
pub mod media;
pub mod property;
pub mod table_entry;

use crate::tables::property::table::PropertyTable;
use crate::tables::{directory::table::DirectoryTable, media::table::MediaTable};
use component::table::ComponentTable;
use file::table::FileTable;
use getset::{Getters, MutGetters};

/// Enum values are derived from this table:
/// https://learn.microsoft.com/en-us/windows/win32/msi/database-tables
///
/// WARN: This is missing many possible tables as seen when checking the above resource. I have
/// only implemented the tables that I believe will be useful for my usecases at this moment.
///
#[derive(Clone, Debug, Default, Getters, MutGetters)]
#[getset(get = "pub", get_mut = "pub(crate)")]
pub struct MsiBuilderTables {
    component: ComponentTable,
    directory: DirectoryTable,
    file: FileTable,
    media: MediaTable,
    property: PropertyTable,
}
