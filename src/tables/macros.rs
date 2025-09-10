#[macro_export]
macro_rules! opt_val {
    ($var:expr) => {
        (if let Some(binding) = &$var {
            msi::Value::from(*binding)
        } else {
            msi::Value::Null
        })
    };
}

#[macro_export]
macro_rules! opt_str_val {
    ($var:expr) => {
        (if let Some(binding) = &$var {
            msi::Value::from(binding.to_string())
        } else {
            msi::Value::Null
        })
    };
}

#[macro_export]
macro_rules! str_val {
    ($var:expr) => {
        msi::Value::from($var.to_string())
    };
}

#[macro_export]
macro_rules! int_val {
    ($var:expr) => {
        msi::Value::from(Into::<i16>::into($var))
    };
}

#[macro_export]
macro_rules! opt_int_val {
    ($var:expr) => {
        (if let Some(binding) = &$var {
            msi::Value::from(Into::<i16>::into(*binding))
        } else {
            msi::Value::Null
        })
    };
}

#[macro_export]
macro_rules! dint_val {
    ($var:expr) => {
        msi::Value::from(Into::<i32>::into($var))
    };
}

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

                fn columns(&self) -> Vec<msi::Column> {
                    $columns
                }
            }
        }
    };
}
