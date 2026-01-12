use anyhow::ensure;

use crate::tables::builder_list::MsiBuilderList;
use crate::tables::builder_table::MsiBuilderTable;
use crate::tables::dao::IsDao;
use crate::types::column::identifier::ToIdentifier;
use crate::types::helpers::id_generator::IdGenerator;
use crate::types::helpers::to_unique_msi_identifier::ToUniqueMsiIdentifier;

pub(crate) trait IdGeneratorBuilderList: MsiBuilderList {
    type GeneratorType: IdGenerator;
    fn add(&mut self, dao: Self::ListValue) -> anyhow::Result<()> {
        if let Some(identifier) = dao.to_unique_msi_identifier() {
            self.generator_mut().add_used_identifier(identifier)?
        }
        ensure!(!self.contains(&dao), "TEMPERROR");
        self.entries_mut().push(dao);
        Ok(())
    }

    fn entry_with_id(
        &self,
        id: &<Self::GeneratorType as IdGenerator>::IdentifierType,
    ) -> Option<&Self::ListValue> {
        self.entries().iter().find(|entry| {
            entry.to_unique_msi_identifier().unwrap() == id.to_identifier()
        })
    }

    fn generator_mut(&mut self) -> &mut Self::GeneratorType;
    fn generate_id(
        &mut self,
    ) -> <Self::GeneratorType as IdGenerator>::IdentifierType {
        self.generator_mut().generate_id()
    }
}
