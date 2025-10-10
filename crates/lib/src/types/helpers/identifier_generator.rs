use std::cell::RefCell;
use std::rc::Rc;
use std::str::FromStr;

use anyhow::Context;

use crate::types::column::identifier::Identifier;
use crate::types::column::identifier::ToIdentifier;
use crate::types::helpers::id_generator::IdentifierGenerator;
#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) struct IdentifierGenerator {
    count: usize,
    // A reference to a vec of all used Identifiers that should not be generated again.
    // These are all identifiers that inhabit a primary_key column.
    used: std::rc::Rc<std::cell::RefCell<Vec<Identifier>>>,
}

impl IdentifierGenerator {
    fn used(&self) -> &std::rc::Rc<std::cell::RefCell<Vec<Identifier>>> {
        &self.used
    }

    fn count(&self) -> usize {
        self.count
    }

    fn count_mut(&mut self) -> &mut usize {
        &mut self.count
    }

    fn generate_id(&mut self, prefix: &str) -> Self::IdentifierType {
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

impl From<std::rc::Rc<std::cell::RefCell<Vec<Identifier>>>>
    for IdentifierGenerator
{
    fn from(value: std::rc::Rc<std::cell::RefCell<Vec<Identifier>>>) -> Self {
        let count = value.borrow().len();
        Self { used: value, count: 0 }
    }
}
