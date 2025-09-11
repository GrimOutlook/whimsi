#[macro_export]
macro_rules! define_generator_table {
    ($var:ident, $columns:expr) => {
        pastey::paste! {
            #[derive(Debug, Clone, PartialEq)]
            pub struct [<$var:camel Table>] {
                entries: Vec<[<$var:camel Dao>]>,
                generator: [<$var:camel IdGenerator>],
            }

            impl MsiBuilderTable for [<$var:camel Table>] {
                type TableValue = [<$var:camel Dao>];

                fn entries(&self) -> &Vec<Self::TableValue> {
                    &self.entries
                }

                fn entries_mut(&mut self) -> &mut Vec<Self::TableValue> {
                    &mut self.entries
                }

                fn name(&self) -> &'static str {
                    stringify!($var)
                }

                fn columns(&self) -> Vec<whimsi_msi::Column> {
                    $columns
                }
            }
        }
    };
}
