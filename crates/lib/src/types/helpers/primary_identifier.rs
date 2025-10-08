use ambassador::delegatable_trait;

use crate::types::column::identifier::Identifier;

/// Denotes an object that contains a unique identifier that should not be allowed to have other
/// copies present in the MSI. Assuring that only 1 object can have that ID in the MSI database at
/// a given time.
#[delegatable_trait]
pub trait PrimaryIdentifier {
    fn primary_identifier(&self) -> Option<Identifier>;
}
