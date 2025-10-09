use default_dir::DefaultDir;
use identifier::Identifier;
use sequence::Sequence;
use version::Version;

use crate::types::column::filename::Filename;

pub mod binary;
pub mod cabinet;
pub mod condition;
pub mod custom_source;
pub mod default_dir;
pub mod double_integer;
pub mod filename;
pub mod formatted;
pub mod formatted_sddl_text;
pub mod guid;
pub mod identifier;
pub mod integer;
pub mod language;
pub mod property;
pub mod reg_path;
pub mod sequence;
pub mod shortcut;
pub mod text;
pub mod version;

pub enum ColumnValue {
    DefaultDir(DefaultDir),
    Filename(Filename),
    Identifier(Identifier),
    Sequence(Sequence),
    Text(String),
    Version(Version),
}
