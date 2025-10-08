use msi::PackageType;

// Main documentation found
// [here](https://learn.microsoft.com/en-us/windows/win32/msi/security-summary) but additional
// information can be found
// [here](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-oleps/bf7aeae8-c47a-4939-9f45-700158dac3bc)
// on page 35 of revision 9 which includes an additional 2 flags.
// TODO: Make these a bitflag as they are described as "flags" in the documentation implying that
// you can use multiple, even though using "Read-only Recommends" and "Read-only Enforced" at the
// same time doesn't seem intended. PasswordProtected does seem like it could go with any of the
// others though.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DocSecurity {
    NoRestrictions,
    PasswordProtected,
    ReadOnlyRecommended,
    ReadOnlyEnforced,
    LockedForAnnotations,
}

impl DocSecurity {
    pub fn from_package_type(package_type: &PackageType) -> Self {
        match package_type {
            PackageType::Installer => Self::ReadOnlyRecommended,
            PackageType::Patch | PackageType::Transform => {
                Self::ReadOnlyEnforced
            }
        }
    }
}
