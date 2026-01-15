use ambassador::delegatable_trait;

#[delegatable_trait]
pub trait IntoMsiValue {
    fn into(&self) -> msi::Value;
}

impl<T: IntoMsiValue> IntoMsiValue for Option<T> {
    fn into(&self) -> msi::Value {
        match self {
            Some(val) => IntoMsiValue::into(val),
            None => msi::Value::Null,
        }
    }
}
