/// [*Reference*](https://learn.microsoft.com/en-us/windows/win32/msi/reglocator-table#type)
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LocatorType {
    item: LocatorItem,
    arch: LocatorArch,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LocatorItem {
    Directory,
    Filename,
    Registry,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LocatorArch {
    _32bit,
    _64bit,
}

impl Into::<msi::Value> for LocatorType {
    fn into(self) -> msi::Value {
        todo!()
    }
}
