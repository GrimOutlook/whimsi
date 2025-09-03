pub(crate) mod builder_table;
pub mod component;
pub mod directory;
pub mod file;
pub(crate) mod macros;
pub mod media;
pub mod meta;
pub mod property;
pub mod table_entry;

use std::io::{Read, Seek, Write};
use std::slice::Iter;

use crate::tables::property::table::PropertyTable;
use crate::tables::{directory::table::DirectoryTable, media::table::MediaTable};
use builder_table::MsiBuilderTable;
use component::table::ComponentTable;
use file::table::FileTable;
use getset::{Getters, MutGetters};
use msi::Package;
use tracing::info;

/// Enum values are derived from this table:
/// https://learn.microsoft.com/en-us/windows/win32/msi/database-tables
///
/// WARN: This is missing many possible tables as seen when checking the above resource. I have
/// only implemented the tables that I believe will be useful for my usecases at this moment.
///
// TODO: Implement enum_dispatch for the MsiBuilderTable trait so that we can just create a `Vec`
// of all tables and call `write_to_package` in a for_each.
#[derive(Clone, Debug, Default, Getters, MutGetters)]
#[getset(get = "pub", get_mut = "pub(crate)")]
pub struct MsiBuilderTables {
    component: ComponentTable,
    directory: DirectoryTable,
    file: FileTable,
    media: MediaTable,
    property: PropertyTable,
}

impl MsiBuilderTables {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Just writes the information stored in each of the table properties to the package tables.
    ///
    /// Information is written based on a predetermined order so that information that doesn't
    /// reference other table information is written first.
    pub(crate) fn write_to_package<F: Read + Write + Seek>(
        &self,
        package: &mut Package<F>,
    ) -> anyhow::Result<()> {
        info!("Writing tables to package");
        self.directory.write_to_package(package)?;
        self.component.write_to_package(package)?;
        self.file.write_to_package(package)?;
        // self.media.write_to_package(package);
        // self.property.write_to_package(package);
        Ok(())
    }
}
