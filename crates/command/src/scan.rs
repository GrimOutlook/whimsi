use std::fs::DirEntry;
#[cfg(target_os = "linux")]
use std::os::unix::fs::MetadataExt;
#[cfg(target_os = "windows")]
use std::os::windows::fs::MetadataExt;
use std::rc::Rc;
use std::str::FromStr;

use anyhow::Context;
use anyhow::Result;
use anyhow::bail;
use camino::Utf8DirEntry;
use camino::Utf8PathBuf;
use flexstr::LocalStr;
use flexstr::local_str;
use itertools::Itertools;
use roxygen::roxygen;
use tracing::debug;
use tracing::info;
use tracing::warn;
use uuid::Uuid;

use crate::models::directory::MsiDirectory;
use crate::models::file::MsiFile;
use crate::models::sequencer::Sequencer;
use crate::traits::identifier::Identifier;
