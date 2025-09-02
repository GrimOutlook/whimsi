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
