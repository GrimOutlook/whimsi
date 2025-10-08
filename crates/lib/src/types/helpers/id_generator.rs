use std::cell::RefCell;
use std::rc::Rc;
use std::str::FromStr;

use anyhow::Context;

use crate::types::column::identifier::Identifier;
use crate::types::column::identifier::ToIdentifier;

pub(crate) trait IdentifierGenerator {
    type IdentifierType: ToIdentifier + FromStr<Err = anyhow::Error>;
    fn id_prefix(&self) -> &str;
    fn used(&self) -> &Rc<RefCell<Vec<Identifier>>>;
    fn count(&self) -> usize;
    fn count_mut(&mut self) -> &mut usize;

    fn generate_id(&mut self) -> Self::IdentifierType {
        loop {
            let new_id = &format!("_{}{}", self.id_prefix(), self.count());
            let new_identifier = Self::IdentifierType::from_str(&new_id)
                .with_context(|| {
                    format!(
                        "[{}] could not be turned into a [{}] identifier",
                        new_id,
                        self.id_prefix()
                    )
                })
                .unwrap();

            let generic_identifier =
                <Self::IdentifierType as ToIdentifier>::to_identifier(
                    &new_identifier,
                );
            if !self.used().borrow().contains(&generic_identifier) {
                return new_identifier;
            }

            *self.count_mut() += 1;
        }
    }

    fn add_used_identifier(
        &mut self,
        identifier: impl ToIdentifier,
    ) -> anyhow::Result<()> {
        let identifier = identifier.to_identifier();
        if self.used().borrow().contains(&identifier) {
            anyhow::bail!("Identifier [{}] is already used", identifier)
        }

        self.used().borrow_mut().push(identifier);
        Ok(())
    }
}
