pub trait MsiBuilderTable: Default {
    type TableValue;

    /// Utilized when creating the MSI using the `msi` crate.
    fn name() -> &'static str;
    fn default_values() -> Vec<Self::TableValue>;
    fn values(&self) -> &Vec<Self::TableValue>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn add(&mut self, dao: Self::TableValue) -> anyhow::Result<()>;
}

#[macro_export]
macro_rules! msitable_boilerplate {
    () => {
        fn values(&self) -> &Vec<Self::TableValue> {
            &self.0
        }
        fn len(&self) -> usize {
            self.0.len()
        }
        fn is_empty(&self) -> bool {
            self.0.is_empty()
        }
    };
}
