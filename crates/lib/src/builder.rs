use std::cell::RefCell;
use std::io::Read;
use std::io::Seek;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::rc::Rc;
use std::str::FromStr;

use anyhow::Context;
use anyhow::bail;
use getset::Getters;
use getset::Setters;
use getset::WithSetters;
use itertools::Itertools;
use msi::Insert;
use msi::Value;
use once_cell::sync::OnceCell;
use once_cell::unsync::Lazy;
use rand::distr::Alphanumeric;
use rand::distr::SampleString;
use tracing::debug;
use tracing::info;
use uuid::Uuid;

use crate::constants::*;
use crate::tables::AdminExecuteSequenceTable;
use crate::tables::MsiTable;
use crate::tables::MsiTableContainer;
use crate::tables::builder_table::MsiTableKind;
use crate::tables::dao::MsiDao;
use crate::tables::meta::MetaInformation;
use crate::types::column::default_dir::DefaultDir;
use crate::types::column::filename::Filename;
use crate::types::column::identifier::Identifier;
use crate::types::column::identifier::ToIdentifier;
use crate::types::column::sequence::Sequence;
use crate::types::helpers::architecture::MsiArchitecture;
use crate::types::helpers::cabinet_info::CabinetInfo;
use crate::types::helpers::cabinets::CabinetHandle;
use crate::types::helpers::cabinets::Cabinets;
use crate::types::helpers::page_count::PageCount;
use crate::types::helpers::security_flag::DocSecurity;
use crate::types::properties::system_folder::SystemFolder;
use crate::types::standard_action::StandardAction;

/// An in-memory representation of the final MSI to be created.
#[derive(Getters, Setters)]
#[getset(get = "pub")]
pub struct MsiBuilder {
    /// Information about the whole package. Tracks both information for
    /// creating the MSI and information that is tracked in the
    /// _SummaryInformation table.
    ///
    /// WARN: The MSI cannot be built without this information being filled
    /// out.
    #[getset(set = "pub")]
    meta: Option<MetaInformation>,

    /// A list of all identifiers used in this MSI. Used to ensure no duplicate
    /// Identifiers are created.
    identifiers: Rc<RefCell<Vec<Identifier>>>,

    /// List of all the tables managed by this builder
    tables: Vec<MsiTableContainer>,
}

impl MsiBuilder {
    /// Build the MSI from all information given to MSIBuilder.
    pub fn build<F: std::io::Read + std::io::Write + std::io::Seek>(
        self,
        mut container: F,
    ) -> anyhow::Result<msi::Package<F>> {
        let Some(ref meta) = self.meta else {
            bail!("Meta information cannot be blank");
        };
        info!("Building MSI");

        let mut package =
            msi::Package::create(msi::PackageType::Installer, container)?;
        self.write_meta_info_to_package(&mut package, meta)?;
        self.write_tables_to_package(&mut package)?;

        info!("Finished building MSI");
        Ok(package)
    }

    pub(crate) fn write_meta_info_to_package<
        F: std::io::Read + std::io::Write + std::io::Seek,
    >(
        &self,
        package: &mut msi::Package<F>,
        meta: &MetaInformation,
    ) -> anyhow::Result<()> {
        let package_type = package.package_type();
        package.set_database_codepage(msi::CodePage::Windows1252);
        let summary_info = package.summary_info_mut();
        summary_info.set_codepage(msi::CodePage::Windows1252);
        summary_info.set_subject(meta.subject());

        if let Some(author) = meta.author() {
            summary_info.set_author(author);
        }
        summary_info.set_languages(meta.languages());
        if let Some(arch) = meta.architecture() {
            summary_info.set_arch(arch.to_string());
        }
        if let Some(comments) = meta.comments() {
            summary_info.set_comments(comments);
        }
        // TODO: Change this after testing. Just trying to make everything
        // exactly the same.
        summary_info.set_creating_application("msitools 0.106");
        summary_info.set_creation_time_to_now();
        summary_info.set_last_save_time_to_now();
        summary_info.set_keywords(meta.keywords());
        summary_info.set_uuid(Uuid::new_v4());
        summary_info.set_word_count(2);
        // TODO: Determine if older versions should be supported.
        // Only support versions after 5.0
        summary_info.set_page_count(PageCount::_5_0 as i32);
        summary_info.set_doc_security(DocSecurity::from_package_type(
            &package_type,
        ) as i32);
        Ok(())
    }

    /// Just writes the information stored in each of the table properties to
    /// the package tables.
    ///
    /// Information is written based on a predetermined order so that
    /// information that doesn't reference other table information is
    /// written first.
    pub(crate) fn write_tables_to_package<
        F: std::io::Read + std::io::Write + std::io::Seek,
    >(
        &self,
        package: &mut msi::Package<F>,
    ) -> anyhow::Result<()> {
        info!("Writing tables to package");
        for table in self.tables() {
            // TODO: Need to make this function and all others that don't
            // operate on an associated type to their own traits.
            table.write_to_package(package)?
        }
        debug!(
            "Wrote tables to MSI: {:?}",
            package.tables().map(|t| t.name()).collect_vec()
        );
        Ok(())
    }

    pub fn table_mut(&mut self, table: MsiTable) -> &mut MsiTableContainer {
        self.tables
            .iter_mut()
            .find(|t| MsiTable::from(*t as &MsiTableContainer) == table)
            .unwrap()
    }

    pub fn table(&self, table: MsiTable) -> &MsiTableContainer {
        self.tables
            .iter()
            .find(|t| MsiTable::from(*t as &MsiTableContainer) == table)
            .unwrap()
    }
}

impl Default for MsiBuilder {
    fn default() -> Self {
        let empty_entries = Rc::new(RefCell::new(Vec::new()));
        Self {
            meta: None,

            // Non-tables that need access to all or generate entity IDs.
            identifiers: empty_entries.clone(),
            tables: Vec::new(),
        }
    }
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum MsiBuilderError {
    #[error(
        "Property with identifier {identifier} not found in Property table"
    )]
    InvalidTargetDirChild { identifier: Identifier },
    #[error("TARGETDIR cannot be a subdirectory")]
    SubRootDirectory,
    #[error(
        "Directory with identifier {identifier} not found in Directory table"
    )]
    DirectoryNotFound { identifier: Identifier },
    #[error("Directory with ID {identifier} already exists in Directory Table")]
    DirectoryIdentifierConflict { identifier: Identifier },
    #[error(
        "Identifier {identifier} already exists for MSI. Identifiers must be unique."
    )]
    IdentifierConflict { identifier: Identifier },
    #[error("No directory name could be found for path [{path}]")]
    NoDirectoryName { path: PathBuf },
    #[error("Invalid directory name found for path [{path}]")]
    InvalidDirectoryName { path: PathBuf },
}
