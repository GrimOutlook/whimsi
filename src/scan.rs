#[cfg(target_os = "linux")]
use std::os::unix::fs::MetadataExt;
#[cfg(target_os = "windows")]
use std::os::windows::fs::MetadataExt;
use std::{fs::DirEntry, rc::Rc, str::FromStr};

use anyhow::{bail, Context, Result};
use camino::{Utf8DirEntry, Utf8PathBuf};
use flexstr::{local_str, LocalStr};
use itertools::Itertools;
use roxygen::roxygen;
use tracing::{debug, info, warn};
use uuid::Uuid;

use crate::{
    models::{directory::Directory, file::File, sequencer::Sequencer},
    traits::identifier::Identifier,
};

const DOT: LocalStr = local_str!(".");
const SOURCEDIR: LocalStr = local_str!("SourceDir");
const TARGETDIR: LocalStr = local_str!("TARGETDIR");
const PROGRAMFILESFOLDER: LocalStr = local_str!("ProgramFilesFolder");
const PROGRAMFILES64FOLDER: LocalStr = local_str!("ProgramFiles64Folder");

// pub(crate) fn scan_paths(
//     config: Rc<MsiConfig>,
//     input_directory: &Utf8PathBuf,
// ) -> anyhow::Result<(Vec<Directory>, Vec<File>)> {
//     info!("Scanning paths to include in the MSI");
//     // Keeps track of the file installation order. The `File` object has a
//     // sequence field that needs to be
//     let mut file_sequencer = Sequencer::new(1);
//     let mut directories = Vec::new();
//     let mut files = Vec::new();
//
//     if config.explicit_files.is_some() {
//         let (scanned_dirs, scanned_files) = &mut add_explicit_path_directories(
//             config.clone(),
//             input_directory,
//             &mut file_sequencer,
//         )?;
//         directories.append(scanned_dirs);
//         files.append(scanned_files);
//     }
//
//     if config.default_files.is_some() {
//         let (scanned_dirs, scanned_files) =
//             &mut add_default_directories(config, input_directory, &mut file_sequencer)?;
//         directories.append(scanned_dirs);
//         files.append(scanned_files);
//     }
//
//     Ok((directories, files))
// }
//
// fn add_explicit_path_directories(
//     _config: Rc<MsiConfig>,
//     _input_directory: &Utf8PathBuf,
//     _file_sequencer: &mut Sequencer,
// ) -> anyhow::Result<(Vec<Directory>, Vec<File>)> {
//     warn!("Sorry! Explicit paths are currently not implemented.");
//     // TODO: Finish implementing explicit path directories.
//     unimplemented!("Explicit paths are currently not supported.");
// }
//
// fn add_default_directories(
//     config: Rc<MsiConfig>,
//     input_directory: &Utf8PathBuf,
//     file_sequencer: &mut Sequencer,
// ) -> anyhow::Result<(Vec<Directory>, Vec<File>)> {
//     debug!(
//         "Adding default directories for input path [{}]",
//         input_directory
//     );
//     let files_section = config
//         .default_files
//         .as_ref()
//         .expect("Failed to get `default_files` section from MsiConfig");
//
//     let mut default_directories = vec![
//         // The value of the DefaultDir column for the root directory entry must
//         // be set to the SourceDir property per [this
//         // section](https://learn.microsoft.com/en-us/windows/win32/msi/directory-table#root-source-directory).
//         // This will be present in every install with a files section.
//         Directory::new(TARGETDIR, None, SOURCEDIR, None),
//     ];
//
//     // Add the Program Files listing if it is included in the config.
//     if let Some(program_files) = &files_section.program_files {
//         default_directories.append(&mut program_files_directory(
//             &config,
//             PROGRAMFILES64FOLDER,
//             input_directory.join(Utf8PathBuf::from_str(program_files).unwrap()),
//         ));
//     };
//
//     // Add the Program Files (x86) listing if it is included in the config.
//     if let Some(program_files_32) = &files_section.program_files_32 {
//         default_directories.append(&mut program_files_directory(
//             &config,
//             PROGRAMFILESFOLDER,
//             input_directory.join(Utf8PathBuf::from_str(program_files_32).unwrap()),
//         ));
//     };
//
//     // Add the Desktop listing if it is included in the config.
//     if let Some(desktop) = &files_section.desktop {
//         default_directories.push(Directory::new(
//             "DesktopFolder".to_string(),
//             Some(TARGETDIR),
//             DOT,
//             Some(input_directory.join(Utf8PathBuf::from_str(desktop).unwrap())),
//         ));
//     };
//
//     let mut directories = default_directories.clone();
//     let mut files = Vec::new();
//     for directory in default_directories {
//         let Some(path) = directory.source() else {
//             debug!("Not scanning directory [{}] for paths", directory.name());
//             continue;
//         };
//         // Scan the current path and append
//         let (scanned_dirs, scanned_files) = &mut scan_path(path, file_sequencer, directory.id())?;
//         directories.append(scanned_dirs);
//         files.append(scanned_files);
//     }
//
//     Ok((directories, files))
// }
//
// fn program_files_directory(
//     config: &Rc<MsiConfig>,
//     program_files_label: LocalStr,
//     source_dir: Utf8PathBuf,
// ) -> Vec<Directory> {
//     let program_folder_uuid = Uuid::as_identifier();
//     let manufacturer_folder_uuid = Uuid::as_identifier();
//     vec![
//         Directory::new(program_files_label.clone(), Some(TARGETDIR), DOT, None),
//         Directory::new(
//             manufacturer_folder_uuid.clone(),
//             Some(program_files_label),
//             config.product_info.manufacturer.to_string(),
//             None,
//         ),
//         Directory::new(
//             program_folder_uuid,
//             Some(manufacturer_folder_uuid),
//             config.product_info.product_name.to_string(),
//             Some(source_dir),
//         ),
//     ]
// }

/// Scans only the given directory. Does not scan directories inside this directory.
///
/// Returns a tuple of vecs. The first holds directories and the second holds files.
pub fn scan_path(path: &Utf8PathBuf) -> Result<(Vec<Utf8DirEntry>, Vec<Utf8DirEntry>)> {
    debug!("Scanning directory path [{}]", path);
    // Get the entries present in the `path` directory.
    let directory_entries = path
        .read_dir_utf8()
        .context(format!("Failed to read directory [{path}]"))?;

    // Get all of the entries that did not return an `Err` when scanned.
    let (ok_entries, errs): (Vec<_>, Vec<_>) = directory_entries.partition_result();
    // If any of them returned an error, short circuit and return that error.
    // May change this behavior based on config if desired in the future.
    if !errs.is_empty() {
        // TODO: Add an `--ignore-file-errors` option to continue on with files that did not
        // produce errors.
        bail!("Failed to read file inside {}", path);
    }

    // Get all of the entries that have a valid file type. We need to check if
    // these are directories so if we can't read that from somewhere we need to
    // exit.
    //
    // Also I shamelessly stole the [implementation] for
    // [`partition_result`](https://docs.rs/itertools/0.14.0/src/itertools/lib.rs.html#3669-3679)
    // in itertools to make this because I needed the entries back out, not the
    // filetypes that have to be checked.
    let (mut found_dir_paths, mut found_file_paths) = (Vec::new(), Vec::new());
    for entry in ok_entries {
        let filetype = entry.file_type()?;
        // Only keep the entries that are either directories or files. We don't
        // care about symlinks or other file types.
        if filetype.is_dir() {
            found_dir_paths.push(entry)
        } else if filetype.is_file() {
            found_file_paths.push(entry)
        } else {
            // TODO: Handle following symlinks.
            // TODO: Look into copying symlink layouts to windows.
            todo!("Symlinks are not currently supported")
        }
    }

    Ok((found_dir_paths, found_file_paths))
}
