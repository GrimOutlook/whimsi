pub(crate) mod builder_table;
pub(crate) mod dao;
pub mod meta;

use msi::Category;
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
            /// If this column is null the installer does not register the
            /// component and the component cannot be removed or repaired by the
            /// installer. This might be intentionally done if the component is
            /// only needed during the installation, such as a custom action
            /// that cleans up temporary files or removes an old product. It may
            /// also be useful when copying data files to a user's computer that
            /// do not need to be registered.
            #[msi_column(kind(string(category = Category::Guid, length = 38)))]
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
            #[msi_column(kind(string(category = Category::Condition, length = 255)))]
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

        /// The Condition table can be used to modify the selection state of any
        /// entry in the Feature table based on a conditional expression.
        Condition {
            /// The identifier of the feature that this condition effects.
            #[msi_column(primary_key, kind(identifier(id_length = "short")))]
            feature_: Identifier,

            /// A conditional install level for the feature in the Feature_
            /// column of this table. The installer sets the install level of
            /// this feature to the level specified in this column if the
            /// expression in the Condition column evaluates to TRUE.
            #[msi_column(primary_key, kind(integer))]
            level: Integer,

            /// If this conditional expression evaluates to TRUE, then the Level
            /// column in the Feature table is set to the conditional install
            /// level.
            ///
            /// The expression in the Condition column should not contain
            /// reference to the installed state of any feature or component.
            /// This is because the expressions in the Condition column are
            /// evaluated before the installer evaluates the installed states of
            /// features and components. Any expression in the Condition table
            /// that attempts to check the installed state of a feature or
            /// component always evaluates to false.
            #[msi_column(kind(string(category = Category::Condition, length = 255)))]
            condition: Condition,
        },

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
            #[msi_column(kind(string(localizable, category = Category::DefaultDir, length = 255)))]
            default_dir: DefaultDir,
        },

        Feature {
            /// The primary key that is used to identify a specific feature
            /// record. The value in this field must not exceed a maximum length
            /// of 38 characters.
            #[msi_column(primary_key, kind(identifier(id_length = "short")))]
            feature: Identifier,

            /// An optional key of a parent record in the same table.
            ///
            /// The key points to the Feature column. If the parent feature is
            /// not selected, then this feature is not installed. A null value
            /// in this field indicates that this feature does not have a parent
            /// and is a root item. The Feature_Parent column must not equal the
            /// Feature column of the same record.
            ///
            /// # Note
            /// The maximum depth of any feature is 16. An error 2701 results if
            /// a feature that exceeds this maximum depth exists.
            #[msi_column(kind(identifier(id_length = "short")), column_name = "Feature_Parent")]
            parent_feature: Option<Identifier>,

            /// A short string of text that identifies a feature.
            ///
            /// This string is listed as an item by the SelectionTree Control of
            /// the Selection Dialog.
            #[msi_column(kind(string(category = Category::Text, length = 64)))]
            title: Option<Text>,

            /// A longer string of text that describes a feature.
            ///
            /// This localizable string is displayed by the Text Control of the
            /// Selection Dialog.
            #[msi_column(kind(string(category = Category::Text, length = 255)))]
            description: Option<Text>,

            /// The number in this field specifies the order in which the
            /// feature is to be displayed in the user interface.
            ///
            /// The value also determines whether or not the feature is
            /// initially displayed expanded or collapsed. If the value is null
            /// or 0 (zero), the record is not displayed.
            /// - If the value is odd, the feature node is expanded initially.
            /// - If the value is even, the feature node is collapsed initially.
            #[msi_column(kind(integer))]
            display: Option<Integer>,

            /// The initial installation level of this feature. Processing the
            /// Condition Table can modify the level value.
            ///
            /// An install level of 0 (zero) disables the item and prevents it
            /// from being displayed. A feature with an installation level of 0
            /// (zero) is not installed during any installation, including
            /// administrative installations. For more information, see the
            /// "Install Level" information in the Remarks section of this
            /// topic.
            #[msi_column(kind(integer))]
            level: Integer,

            /// The Directory_ column specifies the name of a directory that can
            /// be configured by a Selection Dialog.
            ///
            /// Because this field is a key into the Directory Table, the
            /// specified directory must be listed in the first column of the
            /// Directory Table. You must enter a Public Property in this column
            /// to make the directory configurable, and to display a Browse
            /// button on the Selection Dialog.
            #[msi_column(kind(identifier(id_length = "long", foreign_key(table = "Directory"))))]
            directory_: Identifier,

            /// The remote execution option for features that are not installed and for which no feature state request is made by using any of the following properties.
            /// - [ADDLOCAL Property](https://learn.microsoft.com/en-us/windows/win32/msi/addlocal)
            /// - [ADDSOURCE Property](https://learn.microsoft.com/en-us/windows/win32/msi/addsource)
            /// - [ADDDEFAULT Property](https://learn.microsoft.com/en-us/windows/win32/msi/adddefault)
            /// - [COMPADDLOCAL Property](https://learn.microsoft.com/en-us/windows/win32/msi/compaddlocal)
            /// - [COMPADDSOURCE Property](https://learn.microsoft.com/en-us/windows/win32/msi/fileaddlocal)
            /// - [FILEADDLOCAL Property](https://learn.microsoft.com/en-us/windows/win32/msi/fileaddlocal)
            /// - [FILEADDSOURCE Property](https://learn.microsoft.com/en-us/windows/win32/msi/fileaddsource)
            /// - [REMOVE Property](https://learn.microsoft.com/en-us/windows/win32/msi/remove)
            /// - [REINSTALL Property](https://learn.microsoft.com/en-us/windows/win32/msi/reinstall)
            /// - [ADVERTISE Property](https://learn.microsoft.com/en-us/windows/win32/msi/advertise)
            ///
            /// Add the indicated bits to the total value of this column to include a remote execution option.
            /// - If this field is blank, the value defaults to 0 (zero), msidbFeatureAttributesFavorLocal.
            /// - If the feature install level is 0 (zero), or greater than or equal to the current install level, no change is made in the feature state.
            ///
            /// Some attributes are exclusive of each other. Attempting to set
            /// these attributes together on the same feature causes the
            /// installation package to fail Package Validation.
            /// - Do not use msidbFeatureAttributesFavorAdvertise with
            /// msidbFeatureAttributesDisallowAdvertise.
            /// - Do not use msidbFeatureAttributesNoUnsupportedAdvertise with
            /// msidbFeatureAttributesDisallowAdvertise together.
            /// - Do not use msidbFeatureAttributesFollowParent with
            /// msidbFeatureAttributesFavorSource.
            /// - Note that the msidbFeatureAttributesFollowParent and
            /// msidbFeatureAttributesFavorLocal values are mutually exclusive.
            /// If the msidbFeatureAttributesFollowParent value is used, the
            /// msidbFeatureAttributesFavorLocal value is assumed to not exist.
            ///
            /// Note that if a child feature is installed, its parent feature is
            /// also installed. If a parent feature is installed, its child
            /// feature is not necessarily installed unless its
            /// msidbFeatureAttributesFollowParent and
            /// msidbFeatureAttributesUIDisallowAbsent attributes are set. This
            /// hierarchical relationship of the installation of parent and
            /// child features is also used for the GUI installations and
            /// installations that use command-line properties.
            #[msi_column(kind(integer))]
            attributes: FeatureAttributes,
        },

        /// The FeatureComponents table defines the relationship between features and components.
        /// For each feature, this table lists all the components that make up that feature.
        ///
        /// There is a maximum limit of 1600 components per feature.
        FeatureComponents {
            /// An external key into the first column of the Feature table.
            #[msi_column(primary_key, kind(identifier(id_length = "short")))]
            feature_: Identifier,

            /// An external key into the first column of the Component table.
            #[msi_column(primary_key, kind(identifier(id_length = "long")))]
            components_: Identifier,
        },

        /// The Property table contains the property names and values for all
        /// defined properties in the installation. Properties with Null values
        /// are not present in the table.
        Property {
            /// The name of a property.
            #[msi_column(primary_key, kind(identifier(id_length = "long")))]
            property: Identifier,
            /// A localizable string value for the property. This may never be
            /// Null or an empty string.
            #[msi_column(kind(string(length = 0, category = Category::Text)))]
            value: Property,
        },

        // TODO:
        // PublishComponent,
        // File,
        // RemoveFile,
        // MoveFile,
        // DuplicateFile,
        // CreateFolder,
        // Media,
        // Environment,
        // Icon,
        // Binary,
        // MsiFileHash,
        // Shortcut,
        // Registry,
        // RemoveRegistry,
        // Error,
        // InstallUISequence,
        // InstallExecuteSequence,
        // AdminUISequence,
        // AdminExecuteSequence,
        // AdvtExecuteSequence,
        // CustomAction,
        // LaunchCondition,
    }
}
