use crate::{msitable_boilerplate, tables::builder_table::MsiBuilderTable};

use super::dao::ComponentDao;

#[derive(Debug, Clone, Default)]
pub struct ComponentTable(Vec<ComponentDao>);
impl MsiBuilderTable for ComponentTable {
    type TableValue = ComponentDao;

    // Boilderplate needed to access information on the inner object
    msitable_boilerplate!();

    fn name() -> &'static str {
        "Component"
    }

    fn default_values() -> Vec<Self::TableValue> {
        todo!()
    }
    fn add(&mut self, dao: Self::TableValue) -> anyhow::Result<()> {
        self.0.push(dao);
        Ok(())
    }
}
