use std::cell::RefCell;
use std::rc::Rc;
use std::str::FromStr;

use anyhow::Context;

use crate::types::column::identifier::Identifier;
use crate::types::column::identifier::ToIdentifier;

pub(crate) trait IdGenerator {
    type IdentifierType: ToIdentifier + FromStr<Err = anyhow::Error>;
    fn id_prefix(&self) -> &str;
    fn used(&self) -> &Rc<RefCell<Vec<Identifier>>>;
    fn count(&self) -> usize;
    fn count_mut(&mut self) -> &mut usize;

    fn generate_id(&mut self) -> Self::IdentifierType {
        loop {
            let new_id = &format!("{}{}", self.id_prefix(), self.count());
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

#[macro_export]
macro_rules! define_specific_identifier {
    ($id_type:ident) => {
        pastey::paste! {
            #[derive(Clone, Debug, Default, PartialEq, derive_more::Display)]
            pub struct [<$id_type:camel Identifier>]($crate::types::column::identifier::Identifier);

            impl $crate::types::column::identifier::ToIdentifier for [<$id_type:camel Identifier>] {
                fn to_identifier(&self) -> $crate::types::column::identifier::Identifier {
                    self.0.clone()
                }
            }
            impl $crate::types::helpers::to_unique_msi_identifier::ToUniqueMsiIdentifier for [<$id_type:camel Identifier>] {
                fn to_unique_msi_identifier(&self) -> Option<$crate::types::column::identifier::Identifier> {
                    Some(self.0.clone())
                }
            }
        }
    };
}

#[macro_export]
macro_rules! define_specific_identifier_parsing {
    ($id_type:ident) => {
        pastey::paste! {
            impl std::str::FromStr for [<$id_type:camel Identifier>] {
                type Err = anyhow::Error;

                fn from_str(s: &str) -> anyhow::Result<Self> {
                    Ok(Self($crate::types::column::identifier::Identifier::from_str(s)?))
                }
            }
        }
    };
}

#[macro_export]
macro_rules! define_identifier_generator {
    ($id_type:ident) => {

        pastey::paste! {
            #[derive(Debug, Clone, Default, PartialEq)]
            pub(crate) struct [<$id_type:camel IdGenerator>] {
                count: usize,
                used: std::rc::Rc<std::cell::RefCell<Vec<$crate::types::column::identifier::Identifier>>>,
            }

            impl $crate::types::helpers::id_generator::IdGenerator for [<$id_type:camel IdGenerator>] {
                type IdentifierType = [<$id_type:camel Identifier>];

                fn id_prefix(&self) -> &str {
                    $crate::constants::[<$id_type:upper _IDENTIFIER_PREFIX>]
                }

                fn used(&self) -> &std::rc::Rc<std::cell::RefCell<Vec<$crate::types::column::identifier::Identifier>>> {
                    &self.used
                }

                fn count(&self) -> usize {
                    self.count
                }

                fn count_mut(&mut self) -> &mut usize {
                    &mut self.count
                }
            }

            impl From<std::rc::Rc<std::cell::RefCell<Vec<$crate::types::column::identifier::Identifier>>>> for [<$id_type:camel IdGenerator>] {
                fn from(value: std::rc::Rc<std::cell::RefCell<Vec<$crate::types::column::identifier::Identifier>>>) -> Self {
                    let count = value.borrow().len();
                    Self {
                        used: value,
                        count: 0,
                    }
                }
            }
        }
    };
}

#[macro_export]
macro_rules! implement_id_generator_for_table {
    ($table:ident, $generator:ident) => {
        impl $crate::tables::id_generator_builder_list::IdGeneratorBuilderList
            for $table
        {
            type GeneratorType = $generator;

            fn generator_mut(&mut self) -> &mut Self::GeneratorType {
                &mut self.generator
            }
        }
    };
}

#[macro_export]
macro_rules! implement_new_for_id_generator_table {
    ($table:ident, $generator:ident) => {
        impl $table {
            pub fn new(
                identifiers: std::rc::Rc<
                    std::cell::RefCell<
                        Vec<$crate::types::column::identifier::Identifier>,
                    >,
                >,
            ) -> Self {
                let entries = Vec::new();
                let generator = $generator::from(identifiers);
                Self { entries, generator }
            }
        }
    };
}
