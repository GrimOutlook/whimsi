use std::collections::HashMap;
use std::env;

use anyhow::Context;
use camino::Utf8PathBuf;
use flexstr::LocalStr;
use serde::Deserialize;
use serde_inline_default::serde_inline_default;
use serde_with::skip_serializing_none;
use whimsi_lib::tables::lock_permissions::lock_permissions::LockPermissions;
use whimsi_lib::tables::service_install::error_control::ErrorControl;
use whimsi_lib::tables::service_install::service_type::ServiceType;
use whimsi_lib::tables::service_install::start_type::StartType;

#[derive(Deserialize)]
#[serde(rename = "Summary")]
pub(crate) struct SummaryConfigInfo {
    pub(crate) subject: String,
    pub(crate) author: String,
    #[serde(default)]
    pub(crate) comments: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename = "Install")]
pub(crate) struct ServiceInstallConfigInfo {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) executable: String,
    pub(crate) start_time: StartType,
    #[serde(rename = "type")]
    pub(crate) typ: ServiceType,
    pub(crate) interactive: bool,
    pub(crate) error_control: ErrorControl,
    pub(crate) vital: bool,
    pub(crate) control: Option<ServiceControlConfigInfo>,
}

#[derive(Deserialize)]
pub(crate) enum ControlCondition {
    Never,
    Install,
    Uninstall,
    InstallAndUninstall,
}

#[derive(Deserialize)]
#[serde(rename = "Control")]
pub(crate) struct ServiceControlConfigInfo {
    pub(crate) start: ControlCondition,
    pub(crate) stop: ControlCondition,
    pub(crate) remove: ControlCondition,
    pub(crate) wait: bool,
}

#[derive(Deserialize)]
#[serde(rename = "Perm")]
pub(crate) struct Permission {
    pub(crate) user: String,
    pub(crate) level: LockPermissions,
}

#[derive(Deserialize)]
#[serde(rename = "Shortcut")]
pub(crate) struct ShortcutConfigInfo {
    pub(crate) target: String,
    pub(crate) location: String,
    pub(crate) working_directory: Option<String>,
    #[serde(rename = "icon")]
    pub(crate) icon_path: Option<Utf8PathBuf>,
}

#[derive(Deserialize)]
#[serde(rename = "Msi")]
pub(crate) struct MsiConfig {
    pub(crate) summary: SummaryConfigInfo,
    pub(crate) properties: HashMap<String, String>,
    pub(crate) paths: HashMap<Utf8PathBuf, String>,
    pub(crate) permissions: HashMap<String, Permission>,
    pub(crate) shortcuts: Vec<ShortcutConfigInfo>,
    pub(crate) service_installs: Vec<ServiceInstallConfigInfo>,
}

#[cfg(test)]
mod tests {
    use assertables::assert_some;
    use similar_asserts::assert_eq;

    use super::*;
    const TEST_CONFIG: &str = r#"
Msi (
    summary: Summary (
        subject: "Test Program",
        author: "Self",
        comments: Some("Unit test for WHIMSI"),
    ),

    properties: {
        "Manufacturer": "Manny",
        "ProgramName": "Test Program",
        "ProductVersion": "0.0.0",
        "InstallDir": "[[ProgramFiles]]/[Manufacturer]/[ProgramName]/",
    },

    paths: {
        "/tmp/temp_dir": "[InstallDir]"
    },

    permissions: {
        "[InstallDir]/child_dir2/file2.pdf": Perm(user: "Everyone", level: ALL)
    },

    shortcuts: {
        "[InstallDir]/file1.txt": "[[DesktopDir]]/files_shortcut"
    },

    service_installs: [
        Install (
            name: "MyService",
            description: "Description for MyService",
            executable: "[INSTALLDIR]/file1.txt",
            start_time: AutoStart,
            type: OwnProcess,
            interactive: false,
            error_control: Normal,
            vital: true,
            account_name: "LocalSystem",
            control: Some(Control(
                start: Install,
                stop: Uninstall,
                remove: Uninstall,
                wait: true,
            ))
        )
    ]
)
"#;

    #[test]
    fn config_deserializes() {
        let c: MsiConfig = ron::from_str(TEST_CONFIG).unwrap();

        assert_eq!(c.summary.author, "Self");
        assert_eq!(
            c.summary.comments,
            Some("Unit test for WHIMSI".to_string())
        );
    }
}
