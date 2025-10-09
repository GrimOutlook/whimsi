use anyhow::ensure;

use crate::tables::builder_list::MsiBuilderList;
use crate::tables::builder_table::MsiTableKind;
use crate::types::column::identifier::ToIdentifier;
use crate::types::helpers::id_generator::IdentifierGenerator;
use crate::types::helpers::primary_identifier::PrimaryIdentifier;

pub(crate) trait IdentifierGeneratorTable: MsiTableKind {
    type GeneratorType: IdentifierGenerator;
    fn add(&mut self, dao: Self::TableValue) -> anyhow::Result<()> {
        if let Some(identifier) = dao.primary_identifier() {
            self.generator_mut().add_used_identifier(identifier)?
        }
        ensure!(!self.contains(&dao), "TEMPERROR");
        self.entries_mut().push(dao);
        Ok(())
    }

    fn generator_mut(&mut self) -> &mut Self::GeneratorType;
    fn generate_id(
        &mut self,
    ) -> <Self::GeneratorType as IdentifierGenerator>::IdentifierType {
        self.generator_mut().generate_id()
    }
}
