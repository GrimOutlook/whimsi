// TODO: Look into enabling custom directories based on properties.
// I did about an hour of work before realizing it was more effort than I needed to use right now
// while just trying to get basic functionality up and running. From the little work I did I
// learned custom system folders will need to track their own parent, as it can either be None
// (meaning TARGETDIR) or it could be another directory already defined. It will also have to be
// verified that the identifier given for the parent (if not None) and the ID given for the new
// custom system directory is in the `Property` table beforehand as this is where the value for the
// directory Identifier will come from.

use crate::types::column::identifier::Identifier;

#[derive(Clone, Copy, Debug, PartialEq, strum::Display, strum::EnumIter)]
pub enum SystemFolder {
    TARGETDIR,
    ProgramFiles,
}

impl PartialEq<Identifier> for SystemFolder {
    fn eq(&self, other: &Identifier) -> bool {
        other == &Into::<Identifier>::into(*self)
    }
}
