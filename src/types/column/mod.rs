use default_dir::DefaultDir;
use identifier::Identifier;

use super::helpers::filename::Filename;

pub mod default_dir;
pub mod filename;
pub mod identifier;

pub enum ColumnValue {
    DefaultDir(DefaultDir),
    Filename(Filename),
    Identifier(Identifier),
    Text(String),
}
