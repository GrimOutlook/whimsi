pub(crate) mod builder_table;
pub(crate) mod dao;
pub mod meta;

use whimsi_macros::msi_table_list;

use crate as whimsi_lib;
use crate::tables::builder_table::DaoList;
use crate::tables::builder_table::PackageTable;
use crate::tables::dao::MsiDao;
use crate::types::column::binary::Binary;
use crate::types::column::condition::Condition;
// use crate::types::column::custom_source::CustomSource;
use crate::types::column::default_dir::DefaultDir;
use crate::types::column::double_integer::DoubleInteger;
use crate::types::column::filename::Filename;
use crate::types::column::formatted::Formatted;
use crate::types::column::guid::Guid;
use crate::types::column::integer::Integer;
use crate::types::column::language::Language;
use crate::types::column::property::Property;
use crate::types::column::reg_path::RegPath;
use crate::types::column::sequence::Sequence;
// use crate::types::column::shortcut::Shortcut;
use crate::types::column::text::Text;
use crate::types::column::version::Version;
use crate::types::helpers::action_identifier::ActionIdentifier;
use crate::types::helpers::attributes::component::ComponentAttributes;
use crate::types::helpers::attributes::feature::FeatureAttributes;
use crate::types::helpers::attributes::file::FileAttributes;
// use crate::types::helpers::cabinets::CabinetIdentifier;
use crate::types::helpers::custom_action_type::CustomActionType;
use crate::types::helpers::date::Date;
use crate::types::helpers::disk_id::DiskId;
use crate::types::helpers::error_control::ErrorControl;
use crate::types::helpers::key_path::KeyPath;
// use crate::types::helpers::last_sequence::LastSequence;
use crate::types::helpers::locator_type::LocatorType;
// use crate::types::helpers::lock_object::LockObject;
use crate::types::helpers::lock_permission::LockPermission;
use crate::types::helpers::lock_table::LockTable;
use crate::types::helpers::primary_identifier::PrimaryIdentifier;
use crate::types::helpers::registry_root::RegistryRoot;
use crate::types::helpers::service_control_event::ServiceControlEvent;
use crate::types::helpers::service_type::ServiceType;
use crate::types::helpers::show_cmd::ShowCmd;
use crate::types::helpers::start_type::StartType;
use crate::types::helpers::to_msi_value::IntoMsiValue;
use crate::types::standard_action::AdvtAction;
use crate::types::standard_action::StandardAction;

msi_table_list! {
    enum SupportedTable {
        /// The Directory table specifies the directory layout for the product.
        /// Each row of the table indicates a directory both at the source and
        /// the target.
        ///
        /// [_Reference_](https://learn.microsoft.com/en-us/windows/win32/msi/directory-table)
        Directory {

            /// The Directory column contains a unique identifier for a
            /// directory or directory path. This column can contain the name of
            /// a property that is set to the full path of a target directory.
            /// If this column contains a property, the target directory takes
            /// the name specified in the DefaultDir column and takes the parent
            /// directory specified in the Directory_Parent column.
            ///
            /// The source directory always takes the name specified in the
            /// DefaultDir column and takes the parent directory specified in
            /// the Directory_Parent column.
            ///
            /// If the Directory_Parent column is either null or equal to the
            /// value of the Directory column, the Directory column represents a
            /// root target directory. Only one root directory may be specified
            /// in the Directory table.
            #[msi_column(primary_key, kind(identifier(id_length = "long")))]
            directory: Identifier,

            /// This column is a reference to the directory's parent directory.
            /// A record that has a Directory_Parent column equal to null or
            /// equal to the Directory column represents a root directory. The
            /// full path of the parent directory is resolved by reference in
            /// the Directory_Parent column is an external key into the
            /// Directory column. For example, if a folder has a parent
            /// directory named PDIR, the parent directory of PDIR is given in
            /// the Directory_Parent column of the row with PDIR in the
            /// Directory column.
            #[msi_column(kind(identifier(foreign_key(table = "Directory"), id_length = "long")), column_name = "Directory_Parent")]
            parent_directory: Option<Identifier>,

            /// The DefaultDir column contains the directory's name
            /// (localizable) under the parent directory. By default, this is
            /// the name of both the target and source directories. To specify
            /// different source and target directory names, separate the target
            /// and source names with a colon as follows:
            /// [targetname]:[sourcename].
            ///
            /// If the value of the Directory_Parent column is null or is equal
            /// to the Directory column, the DefaultDir column specifies the
            /// name of a root source directory.
            ///
            /// For a non-root source directory, a period (.) entered in the
            /// DefaultDir column for the source directory name or the target
            /// directory name indicates the directory should be located in its
            /// parent directory without a subdirectory.
            ///
            /// The directory names in this column may be formatted as short
            /// filename | long filename pairs.
            #[msi_column(kind(string(localizable, category = msi::Category::DefaultDir, length = 255)))]
            default_dir: DefaultDir,
        },

        /// The Component table lists components
        ///
        /// [_Reference_](https://learn.microsoft.com/en-us/windows/win32/msi/component-table)
        Component {
            /// Identifies the component record.
            ///
            /// Primary table key.
            #[msi_column(primary_key, kind(identifier(id_length = "long")))]
            component: Identifier,

            /// A string GUID unique to this component, version, and language.
            ///
            /// Note that the letters of these GUIDs must be uppercase.
            /// Utilities such as GUIDGEN can generate GUIDs containing
            /// lowercase letters. The lowercase letters must be changed to
            /// uppercase to make these valid component code GUIDs.
            ///
            /// If this column is null the installer does not register the
            /// component and the component cannot be removed or repaired by the
            /// installer. This might be intentionally done if the component is
            /// only needed during the installation, such as a custom action
            /// that cleans up temporary files or removes an old product. It may
            /// also be useful when copying data files to a user's computer that
            /// do not need to be registered.
            #[msi_column(kind(string(category = msi::Category::Guid, length = 38)))]
            component_id: Option<Guid>,

            /// External key of an entry in the Directory table. This is a
            /// property name whose value contains the actual path, which can be
            /// set either by the AppSearch action or with the default setting
            /// obtained from the Directory table.
            ///
            /// Developers must avoid authoring components that place files into
            /// one of the User Profile folders. These files would not be
            /// available to all users in multi-user situations and could cause
            /// the installer to permanently view the component as requiring
            /// repair.
            ///
            /// External key to column one of the Directory table.
            #[msi_column(kind(identifier(id_length = "long", foreign_key(table = "Directory"))))]
            directory_: Identifier,

            /// Specifies options for remote execution.
            #[msi_column(kind(integer))]
            attributes: ComponentAttributes,

            /// This column contains a conditional statement that can control
            /// whether a component is installed. If the condition is null or
            /// evaluates to true, then the component is enabled. If the
            /// condition evaluates to False, then the component is disabled and
            /// is not installed.
            ///
            /// The Condition field enables or disables a component only during
            /// the CostFinalize action. To enable or disable a component after
            /// CostFinalize, you must use a custom action or the DoAction
            /// ControlEvent to call MsiSetComponentState.
            ///
            /// Note that unless the Transitive bit in the Attributes column is
            /// set for a component, the component remains enabled once
            /// installed even if the conditional statement in the Condition
            /// column later evaluates to False on a subsequent maintenance
            /// installation of the product.
            ///
            /// The Condition column in the Component table accepts conditional
            /// expressions containing references to the installed states of
            /// features and components. For information on the syntax of
            /// conditional statements, see Conditional Statement Syntax.
            #[msi_column(kind(string(category = msi::Category::Condition, length = 255)))]
            condition: Option<Condition>,

            /// This value points to a file or folder belonging to the component
            /// that the installer uses to detect the component. Two components
            /// cannot share the same key path value. The value in this column
            /// is also the path returned by the MsiGetComponentPath function.
            ///
            /// If the value is not null, then KeyPath is either a primary key
            /// into the Registry, ODBCDataSource, or File tables depending upon
            /// the Attribute value. If KeyPath is null, then the folder of the
            /// Directory_ column is used as the key path.
            ///
            /// Because folders created by the installer are deleted when they
            /// become empty, you must author an entry into the CreateFolder
            /// table to install a component that consists of an empty folder.
            ///
            /// Note that if a Windows Installer component contains a file or
            /// registry key that is protected by Windows Resource Protection
            /// (WRP) or a file that is protected by Windows File Protection
            /// (WFP), this resource must be used as the KeyPath for the
            /// component. In this case, Windows Installer does not install,
            /// update, or remove the component. You should not include any
            /// protected resources in an installation package. Instead, you
            /// should use the supported resource replacement mechanisms for
            /// Windows Resource Protection. For more information, see Using
            /// Windows Installer and Windows Resource Protection.

            #[msi_column(kind(identifier(id_length = "long")))]
            key_path: Option<KeyPath>,
        },
    }
}
