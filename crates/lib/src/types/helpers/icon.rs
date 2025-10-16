use std::path::PathBuf;

use crate::tables::icon::table::IconIdentifier;

#[derive(Clone, Debug, getset::Getters, derive_more::Constructor)]
#[getset(get = "pub(crate)")]
pub(crate) struct IconInfo {
    path: PathBuf,
    identifier: IconIdentifier,
}
