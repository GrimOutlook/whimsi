use crate::types::standard_action::AdvtAction;
use crate::types::standard_action::StandardAction;

// Found by inspecting MSIs made by other tools. Couldn't find official
// documentation on these values.
pub const CABINET_MAX_LEN: usize = 255;
pub const CONDITION_MAX_LEN: usize = 255;
pub const DEFAULT_DIR_MAX_LEN: usize = 255;
pub const DISK_PROMPT_MAX_LEN: usize = 64;
pub const FILENAME_MAX_LEN: usize = 255;
pub const GUID_MAX_LEN: usize = 38;
pub const DEFAULT_IDENTIFIER_MAX_LEN: usize = 72;
pub const LANGUAGE_MAX_LEN: usize = 20;
pub const SOURCE_MAX_LEN: usize = 72;
pub const VERSION_MAX_LEN: usize = 72;
pub const VOLUME_LABEL_MAX_LEN: usize = 32;
pub const TITLE_MAX_LEN: usize = 64;
pub const DESCRIPTION_MAX_LEN: usize = 255;
// I assume this means, unbounded?
pub const PROPERTY_VALUE_MAX_LEN: usize = 0;
pub const REGISTRY_VALUE_MAX_LEN: usize = 0;
pub const REGISTRY_NAME_MAX_LEN: usize = 255;
pub const REGPATH_MAX_LEN: usize = 255;

// Found here: https://learn.microsoft.com/en-us/windows/win32/msi/filename
pub const SHORT_FILENAME_MAX_LEN: usize = 8;
// TODO: Move filename invalid character array list here.

// Found here: https://learn.microsoft.com/en-us/windows/win32/msi/feature-table
pub const FEATURE_IDENTIFIER_MAX_LEN: usize = 38;

// Found here: https://learn.microsoft.com/en-us/windows/win32/msi/media-table
pub const LAST_SEQUENCE_MIN: usize = 0;
pub const LAST_SEQUENCE_MAX: usize = 32767;

// Found here: https://learn.microsoft.com/en-us/windows/win32/msi/media-table
pub const DISK_ID_MIN: usize = 1;

// Automatically generated Identifier prefixes
pub const CABINET_IDENTIFIER_PREFIX: &str = "CABINET_";
pub const COMPONENT_IDENTIFIER_PREFIX: &str = "COMPONENT_";
pub const DIRECTORY_IDENTIFIER_PREFIX: &str = "DIRECTORY_";
pub const FEATURE_IDENTIFIER_PREFIX: &str = "FEATURE_";
pub const FILE_IDENTIFIER_PREFIX: &str = "FILE_";
pub const MEDIA_IDENTIFIER_PREFIX: &str = "MEDIA_";
pub const PROPERTY_IDENTIFIER_PREFIX: &str = "PROPERTY_";
pub const REGISTRY_IDENTIFIER_PREFIX: &str = "REGISTRY_";
pub const SIGNATURE_IDENTIFIER_PREFIX: &str = "SIGNATURE_";
pub const BINARY_IDENTIFIER_PREFIX: &str = "BINARY_";
pub const SERVICEINSTALL_IDENTIFIER_PREFIX: &str = "SERVICEINST_";
pub const SERVICECONTROL_IDENTIFIER_PREFIX: &str = "SERVICECTRL_";
pub const SHORTCUT_IDENTIFIER_PREFIX: &str = "SHORTCUT_";
pub const ICON_IDENTIFIER_PREFIX: &str = "ICON_";

// Default identifiers
pub const DEFAULT_CABINET_IDENTIFIER: &str = "DEFAULT_CABINET";
pub const DEFAULT_FEATURE_IDENTIFIER: &str = "DEFAULT_FEATURE";
pub const DEFAULT_MEDIA_IDENTIFIER: &str = "DEFAULT_MEDIA";

// Default feature information. Picked by inspecting MSIs to see what the
// defaults were since I never changed any of those settings.
pub const DEFAULT_FEATURE_DISPLAY: i16 = 2;
pub const DEFAULT_FEATURE_LEVEL: i16 = 1;
pub const DEFAULT_FEATURE_TITLE: &str = "Default Feature";

// Default actions for the action tables
pub const ADVT_EXECUTE_SEQUENCE_DEFAULT_ACTIONS: &[AdvtAction] = &[
    AdvtAction::InstallValidate,
    AdvtAction::CostInitialize,
    AdvtAction::CostFinalize,
    AdvtAction::InstallFinalize,
    AdvtAction::PublishFeatures,
    AdvtAction::PublishProduct,
];
pub const ADMIN_EXECUTE_SEQUENCE_DEFAULT_ACTIONS: &[StandardAction] = &[
    StandardAction::CostInitialize,
    StandardAction::FileCost,
    StandardAction::CostFinalize,
    StandardAction::InstallValidate,
    StandardAction::InstallInitialize,
    StandardAction::InstallAdminPackage,
    StandardAction::InstallFiles,
    StandardAction::InstallFinalize,
];
pub const ADMIN_UI_SEQUENCE_DEFAULT_ACTIONS: &[StandardAction] = &[
    StandardAction::CostInitialize,
    StandardAction::FileCost,
    StandardAction::CostFinalize,
    StandardAction::ExecuteAction,
];
pub const INSTALL_EXECUTE_SEQUENCE_DEFAULT_ACTIONS: &[StandardAction] = &[
    StandardAction::InstallValidate,
    StandardAction::FileCost,
    StandardAction::CostInitialize,
    StandardAction::CostFinalize,
    StandardAction::InstallInitialize,
    StandardAction::RegisterUser,
    StandardAction::RegisterProduct,
    StandardAction::PublishFeatures,
    StandardAction::PublishProduct,
    StandardAction::InstallFinalize,
];
pub const INSTALL_UI_SEQUENCE_DEFAULT_ACTIONS: &[StandardAction] = &[
    StandardAction::CostInitialize,
    StandardAction::FileCost,
    StandardAction::CostFinalize,
    StandardAction::ExecuteAction,
];
