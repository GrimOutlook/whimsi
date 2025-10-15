use crate::tables::builder_list_entry::MsiBuilderListEntry;
use crate::tables::dao::IsDao;
use crate::tables::lock_permissions::lock_object::LockObject;
use crate::tables::lock_permissions::lock_permissions::LockPermissions;
use crate::tables::property::property_text::PropertyText;
use crate::types::column::condition::Condition;
use crate::types::column::formatted::Formatted;
use crate::types::column::formatted_sddl_text::FormattedSddlText;
use crate::types::column::identifier::Identifier;
use crate::types::column::identifier::ToIdentifier;
use crate::types::helpers::to_msi_value::ToMsiOptionalValue;
use crate::types::helpers::to_unique_msi_identifier::ToUniqueMsiIdentifier;

#[derive(Clone, Debug)]
pub struct LockPermissionsDao {
    lock_object: LockObject,
    table: String,
    domain: Option<Formatted>,
    user: Formatted,
    permission: Option<LockPermissions>,
}

impl LockPermissionsDao {
    pub fn new(
        lock_object: LockObject,
        user: Formatted,
        permission: LockPermissions,
    ) -> LockPermissionsDao {
        LockPermissionsDao {
            lock_object: lock_object.clone(),
            table: lock_object.table().to_string(),
            domain: None,
            user,
            permission: Some(permission),
        }
    }
}

impl IsDao for LockPermissionsDao {
    fn to_row(&self) -> Vec<whimsi_msi::Value> {
        vec![
            self.lock_object.to_identifier().into(),
            self.lock_object.table().into(),
            self.domain.to_optional_value(),
            self.user.clone().into(),
            self.permission.to_optional_value(),
        ]
    }
}

impl MsiBuilderListEntry for LockPermissionsDao {
    fn conflicts(&self, other: &Self) -> bool {
        self.lock_object == other.lock_object
            && self.domain == other.domain
            && self.user == other.user
    }
}

impl ToUniqueMsiIdentifier for LockPermissionsDao {
    fn to_unique_msi_identifier(&self) -> Option<Identifier> {
        None
    }
}
