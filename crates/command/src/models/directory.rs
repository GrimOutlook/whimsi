use std::fs::DirEntry;
use std::os::unix::process::parent_id;

use anyhow::Context;
use anyhow::Result;
use anyhow::bail;
use camino::Utf8PathBuf;
use derive_new::new;
use flexstr::LocalStr;
use getset::Getters;
use itertools::Itertools;
use tracing::debug;
use uuid::Uuid;

use super::file::MsiFile;
use crate::traits::identifier::Identifier;

/// # [Directory](https://learn.microsoft.com/en-us/windows/win32/msi/directory-table)
///
/// This structure tracks directories that are created and interacted with by
/// the installing MSI.
///
/// ## Properties
///
/// - `id` A unique identifier for a directory or directory path.
/// - `parent_id` The ID of the directory that contains this directory. This is
///   a string and not a `PathBuf` because files can have a property based
///   parent such as `ProgramFiles`, `Desktop`, or `TARGETDIR`.
/// - `name` What the directory will be named (localizable) on the target
///   system.
/// - `source` Path to this directory on the system when generating the MSI.
///   This is optional because some of the default paths do not need to specify
///   a source, such as `DesktopFolder` and `ProgramFiles`, they are simply used
///   in the hierarchy.
#[derive(Clone, Debug, Getters)]
#[getset(get = "pub")]
pub(crate) struct MsiDirectory {
    id: LocalStr,
    parent_id: Option<LocalStr>,
    name: LocalStr,
    source: Option<Utf8PathBuf>,

    // Information not required for the MSI Db
    files: Vec<MsiFile>,
    directories: Vec<MsiDirectory>,
}

impl MsiDirectory {
    pub fn new(parent_id: &str, source: &Utf8PathBuf) -> Result<Self> {
        let id = Uuid::as_identifier();
        let (directories, files) = MsiDirectory::from_path(&id, source)?;
        Ok(MsiDirectory {
            id,
            parent_id: Some(parent_id.into()),
            name: source
                .file_name()
                .expect("Filename somehow ends with '..'. Ending in pure confusion.")
                .into(),
            source: Some(source.clone()),
            files,
            directories,
        })
    }

    pub fn from_path(
        parent: &str,
        path: &Utf8PathBuf,
    ) -> Result<(Vec<MsiDirectory>, Vec<MsiFile>)> {
        let (directories, files) = Self::scan_path(parent, path)?;
        let msi_directories = directories
            .iter()
            .map(|d| MsiDirectory::new(parent, &d.to_path_buf()))
            .collect::<Result<Vec<_>>>()?;
        let msi_files =
            files.iter().map(MsiFile::new).collect::<Result<Vec<_>>>()?;
        Ok((msi_directories, msi_files))
    }

    /// Scans only the given directory. Does not scan directories inside this
    /// directory.
    ///
    /// Returns a tuple of vecs. The first holds directories and the second
    /// holds files.
    pub(super) fn scan_path(
        parent: &str,
        path: &Utf8PathBuf,
    ) -> Result<(Vec<Utf8PathBuf>, Vec<Utf8PathBuf>)> {
        debug!("Scanning directory path [{}]", path);
        // Get the entries present in the `path` directory.
        let directory_entries = path
            .read_dir_utf8()
            .context(format!("Failed to read directory [{path}]"))?;

        // Get all of the entries that did not return an `Err` when scanned.
        let (ok_entries, errs): (Vec<_>, Vec<_>) =
            directory_entries.partition_result();
        // If any of them returned an error, short circuit and return that
        // error. May change this behavior based on config if desired in
        // the future.
        if !errs.is_empty() {
            // TODO: Add an `--ignore-file-errors` option to continue on with
            // files that did not produce errors.
            bail!("Failed to read file inside {}", path);
        }

        // Get all of the entries that have a valid file type. We need to check
        // if these are directories so if we can't read that from
        // somewhere we need to exit.
        //
        // Also I shamelessly stole the [implementation] for
        // [`partition_result`](https://docs.rs/itertools/0.14.0/src/itertools/lib.rs.html#3669-3679)
        // in itertools to make this because I needed the entries back out, not
        // the filetypes that have to be checked.
        let (mut found_dir_paths, mut found_file_paths) =
            (Vec::new(), Vec::new());
        for entry in ok_entries {
            let filetype = entry.file_type()?;
            // Only keep the entries that are either directories or files. We
            // don't care about symlinks or other file types.
            if filetype.is_dir() {
                found_dir_paths.push(entry.into_path())
            } else if filetype.is_file() {
                found_file_paths.push(entry.into_path())
            } else {
                // TODO: Handle following symlinks.
                // TODO: Look into copying symlink layouts to windows.
                todo!("Symlinks are not currently supported")
            }
        }

        Ok((found_dir_paths, found_file_paths))
    }

    pub fn flatten_directories(&self) -> Vec<MsiDirectory> {
        let mut dirs = self.directories.clone();
        self.directories
            .iter()
            .for_each(|dir| dirs.extend(dir.flatten_directories()));
        dirs
    }

    pub fn flatten_files(&self) -> Vec<MsiFile> {
        let mut files = self.files.clone();
        self.directories
            .iter()
            .for_each(|dir| files.extend(dir.flatten_files()));
        files
    }
}

#[cfg(test)]
mod test {
    use std::error::Error;

    use assert_fs::NamedTempFile;
    use assert_fs::TempDir;
    use assert_fs::prelude::*;
    use assertables::*;
    use camino::Utf8Path;
    use camino::Utf8PathBuf;

    use super::MsiDirectory;

    fn setup_test_directory() -> Result<TempDir, Box<dyn Error>> {
        let root = TempDir::with_prefix("root-")?;
        let dir_1 = root.child("child1");
        dir_1.create_dir_all()?;
        let dir_2 = root.child("child2");
        dir_2.create_dir_all()?;
        let file_1 = root.child("file_1.txt");
        file_1.touch()?;
        let file_2 = dir_1.child("file2.pdf");
        file_2.touch()?;
        Ok(root)
    }

    #[test]
    fn new() {
        // Setup the temporary filesystem
        let temp =
            setup_test_directory().expect("Failed to create test directory");

        const PARENT_ID: &str = "TARGETDIR";

        // Run the actual function under test
        let msi_directory = MsiDirectory::new(
            PARENT_ID,
            &Utf8Path::from_path(&temp).unwrap().into(),
        )
        .unwrap();

        // Validate the results
        assert_eq!(msi_directory.parent_id().clone().unwrap(), PARENT_ID);
        assert_eq!(
            msi_directory.directories().len(),
            2,
            "Incorrect directory count found for {temp:?}"
        );
        assert_eq!(
            msi_directory.flatten_directories().len(),
            2,
            "Incorrect recursive directory count found for {temp:?}"
        );
        assert_eq!(
            msi_directory.files().len(),
            1,
            "Incorrect file count for {temp:?}"
        );
        assert_eq!(
            msi_directory.flatten_files().len(),
            2,
            "Incorrect recursive file count for {temp:?}"
        );
        assert_all!(msi_directory.directories().iter(), |d: &MsiDirectory| &d
            .parent_id()
            .clone()
            .unwrap()
            == msi_directory.id())
    }
}
