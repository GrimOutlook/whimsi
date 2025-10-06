use attributes::Attributes;
use default_dir::DefaultDir;
use identifier::Identifier;
use sequence::Sequence;
use version::Version;

use crate::types::column::filename::Filename;

pub mod attributes;
pub mod binary;
pub mod condition;
pub mod custom_source;
pub mod default_dir;
pub mod filename;
pub mod formatted;
pub mod formatted_sddl_text;
pub mod guid;
pub mod identifier;
pub mod reg_path;
pub mod sequence;
pub mod shortcut;
pub mod version;

pub enum ColumnValue {
    Attibutes(Attributes),
    DefaultDir(DefaultDir),
    Filename(Filename),
    Identifier(Identifier),
    Sequence(Sequence),
    Text(String),
    Version(Version),
}
