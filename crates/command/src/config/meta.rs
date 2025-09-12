use std::env;

use anyhow::Context;
use camino::Utf8PathBuf;
use serde::Deserialize;
use serde_inline_default::serde_inline_default;

#[serde_inline_default] // This must come before deriving serde modules.
#[derive(Deserialize)]
#[serde(rename = "meta")]
pub(crate) struct MetaConfig {
    /// Directory that relative paths are appended to to get the full path. Defaults to the CWD.
    #[serde_inline_default(MetaConfig::current_dir())]
    working_directory: Utf8PathBuf,
    /// Where the data for the MSI is to be written. This should include the filename.
    ///
    /// Example: my-msi.msi
    output_path: Utf8PathBuf,
}

impl MetaConfig {
    fn current_dir() -> Utf8PathBuf {
        env::current_dir()
            .context("Could not get the current working directory")
            .unwrap()
            .try_into()
            .context("Current working directory contains non-UTF8 characters.")
            .unwrap()
    }
}
