use attributes::Attributes;
use default_dir::DefaultDir;
use identifier::Identifier;
use sequence::Sequence;
use version::Version;

use super::helpers::filename::Filename;

pub mod attributes;
pub mod condition;
pub mod default_dir;
pub mod filename;
pub mod guid;
pub mod identifier;
pub mod integer;
pub mod sequence;
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
