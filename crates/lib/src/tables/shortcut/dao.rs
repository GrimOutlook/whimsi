use anyhow::Context;

use crate::tables::builder_list_entry::MsiBuilderListEntry;
use crate::tables::component::table::ComponentIdentifier;
use crate::tables::dao::IsDao;
use crate::tables::directory::directory_identifier::DirectoryIdentifier;
use crate::tables::icon::table::IconIdentifier;
use crate::tables::shortcut::table::ShortcutIdentifier;
use crate::types::column::condition::Condition;
use crate::types::column::filename::Filename;
use crate::types::column::formatted::Formatted;
use crate::types::column::guid::Guid;
use crate::types::column::identifier::Identifier;
use crate::types::column::identifier::ToIdentifier;
use crate::types::column::shortcut::Shortcut;
use crate::types::helpers::to_msi_value::ToMsiOptionalValue;
use crate::types::helpers::to_unique_msi_identifier::ToUniqueMsiIdentifier;

#[derive(Debug, Clone, PartialEq, getset::Getters, getset::WithSetters)]
#[getset(get = "pub(crate)", set_with = "pub(crate)")]
pub struct ShortcutDao {
    identifier: ShortcutIdentifier,
    component_: ComponentIdentifier,
    name: Filename,
    directory_: DirectoryIdentifier,
    target: Shortcut,
    arguments: Option<Formatted>,
    description: Option<String>,
    // TODO: Make this a bitflag. Has a lot of options so I'm going to punt it
    // off to later.
    hotkey: Option<i16>,
    icon_: Option<IconIdentifier>,
    icon_index: Option<i16>,
    show_cmd: Option<i16>,
    // TODO: Determine if this is actually an `Identifier` type. The
    // documentation for this field says that the "Windows format [can be
    // used] to reference environment variables, for example %USERPROFILE%"
    // but the `Identifier` type does not allow `%`.
    wk_dir: Option<Identifier>,
    display_resource_dll: Option<Formatted>,
    display_resource_id: Option<i16>,
    description_resource_dll: Option<Formatted>,
    description_resource_id: Option<i16>,
}

impl ShortcutDao {
    pub fn new(
        shortcut_id: ShortcutIdentifier,
        directory_id: DirectoryIdentifier,
        name: Filename,
        component_id: ComponentIdentifier,
        target: Shortcut,
    ) -> ShortcutDao {
        ShortcutDao {
            identifier: shortcut_id,
            directory_: directory_id,
            name,
            component_: component_id,
            target,
            arguments: None,
            description: None,
            hotkey: None,
            icon_: None,
            icon_index: None,
            show_cmd: None,
            wk_dir: None,
            display_resource_dll: None,
            display_resource_id: None,
            description_resource_dll: None,
            description_resource_id: None,
        }
    }
}

impl IsDao for ShortcutDao {
    fn to_row(&self) -> Vec<whimsi_msi::Value> {
        vec![
            self.identifier.to_identifier().into(),
            self.directory_.to_identifier().into(),
            self.name.clone().into(),
            self.component_.to_identifier().into(),
            self.target.clone().into(),
            self.arguments.to_optional_value(),
            self.description.to_optional_value(),
            self.hotkey.to_optional_value(),
            self.icon_.to_optional_value(),
            self.icon_index.to_optional_value(),
            self.show_cmd.to_optional_value(),
            self.wk_dir.to_optional_value(),
            self.display_resource_dll.to_optional_value(),
            self.display_resource_id.to_optional_value(),
            self.description_resource_dll.to_optional_value(),
            self.description_resource_id.to_optional_value(),
        ]
    }
}
impl MsiBuilderListEntry for ShortcutDao {
    fn conflicts(&self, other: &Self) -> bool {
        self.identifier == other.identifier
    }
}

impl ToUniqueMsiIdentifier for ShortcutDao {
    fn to_unique_msi_identifier(&self) -> Option<Identifier> {
        self.identifier.to_unique_msi_identifier()
    }
}
