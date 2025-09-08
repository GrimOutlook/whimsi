use anyhow::ensure;

use crate::{
    tables::{
        builder_list::MsiBuilderList, builder_table::MsiBuilderTable,
        dao::IsDao,
    },
    types::{
        column::identifier::{ToIdentifier, ToOptionalIdentifier},
        helpers::id_generator::IdGenerator,
    },
};

pub(crate) trait IdGeneratorBuilderList: MsiBuilderList {
    type GeneratorType: IdGenerator;
    fn add(&mut self, dao: Self::ListValue) -> anyhow::Result<()> {
        if let Some(identifier) = dao.to_optional_identifier() {
            self.generator_mut().add_used_identifier(identifier)?
        }
        ensure!(!self.contains(&dao), "TEMPERROR");
        self.entries_mut().push(dao);
        Ok(())
    }

    fn generator_mut(&mut self) -> &mut Self::GeneratorType;
    fn generate_id(
        &mut self,
    ) -> <Self::GeneratorType as IdGenerator>::IdentifierType {
        self.generator_mut().generate_id()
    }
}
