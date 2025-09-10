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
    enums::system_folder::SystemFolder,
    models::{directory::MsiDirectory, file::MsiFile, sequencer::Sequencer},
    traits::identifier::Identifier,
};
