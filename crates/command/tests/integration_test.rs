use assert_cmd::Command;
use assert_fs::TempDir;
use assert_fs::prelude::FileTouch;
use assert_fs::prelude::FileWriteStr;
use assert_fs::prelude::PathChild;
use assert_fs::prelude::PathCreateDir;

#[test]
fn test_config_output() {
    // path/to/temp_dir/
    // | - file1.txt
    // | child_dir1/
    // | child_dir2/
    //   | - file2.pdf
    let test_temp_dir = TempDir::new().unwrap();
    let source_temp_dir = test_temp_dir.child("source");
    source_temp_dir.create_dir_all().unwrap();

    let child_dir1 = source_temp_dir.child("child_dir1");
    child_dir1.create_dir_all().unwrap();
    let child_dir2 = source_temp_dir.child("child_dir2");
    child_dir2.create_dir_all().unwrap();
    let file_1 = source_temp_dir.child("file1.txt");
    file_1.touch().unwrap();
    let file_2 = child_dir2.child("file2.pdf");
    file_2.touch().unwrap();
    let source_temp_dir_path = source_temp_dir.path();

    let command_temp_dir = test_temp_dir.child("command");
    command_temp_dir.create_dir_all().unwrap();

    let config = r#"
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
        "InstallDir": "[[ProgramFilesFolder]]/[Manufacturer]/[ProgramName]/",
    },

    paths: {
        "%%temp_dir_path%%/": "[InstallDir]"
    },

    permissions: {
        "[InstallDir]/child_dir2/file2.pdf": Perm(user: "Everyone", level: ALL)
    },

    shortcuts: {
        "[InstallDir]/file1.txt": "[[DesktopFolder]]/files_shortcut"
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
"#
    .replace("%%temp_dir_path%%", &source_temp_dir_path.to_string_lossy());

    let tmp_config = command_temp_dir.child("config.ron");
    tmp_config.write_str(&config).unwrap();
    let tmp_output = command_temp_dir.child("output.msi");
    let output_path = &tmp_output.path().to_string_lossy();

    let mut base_command =
        Command::cargo_bin(assert_cmd::crate_name!()).unwrap();
    let command = base_command.args([
        "build",
        "--config",
        &tmp_config.path().to_string_lossy(),
        "--output",
        output_path,
    ]);

    let assert = command.assert();
    assert.success();

    let mut verifiaction_command = Command::new("msiinfo");
    let command =
        verifiaction_command.args(["export", "-s", output_path, "Directory"]);
    command.assert().stdout( "\
CREATE TABLE `Directory` (`Directory` CHAR(72) NOT NULL, `Directory_Parent` CHAR(72), `DefaultDir` CHAR(255) NOT NULL LOCALIZABLE PRIMARY KEY `Directory`)
INSERT INTO `Directory` (`Directory`, `Directory_Parent`, `DefaultDir`) VALUES ('DesktopFolder', 'TARGETDIR', '.')
INSERT INTO `Directory` (`Directory`, `Directory_Parent`, `DefaultDir`) VALUES ('ProgramFilesFolder', 'TARGETDIR', '.')
INSERT INTO `Directory` (`Directory`, `DefaultDir`) VALUES ('TARGETDIR', 'SourceDir')
INSERT INTO `Directory` (`Directory`, `Directory_Parent`, `DefaultDir`) VALUES ('_DIRECTORY_0', 'ProgramFilesFolder', 'Manny')
INSERT INTO `Directory` (`Directory`, `Directory_Parent`, `DefaultDir`) VALUES ('_DIRECTORY_1', '_DIRECTORY_0', 'Test Program')
INSERT INTO `Directory` (`Directory`, `Directory_Parent`, `DefaultDir`) VALUES ('_DIRECTORY_2', '_DIRECTORY_1', 'child_dir2')
INSERT INTO `Directory` (`Directory`, `Directory_Parent`, `DefaultDir`) VALUES ('_DIRECTORY_3', '_DIRECTORY_1', 'child_dir1')
",
    );

    let mut verifiaction_command = Command::new("msiinfo");
    let command =
        verifiaction_command.args(["export", "-s", output_path, "File"]);
    command.assert().stdout( "\
CREATE TABLE `File` (`File` CHAR(72) NOT NULL, `Component_` CHAR(72) NOT NULL, `FileName` CHAR(255) NOT NULL LOCALIZABLE, `FileSize` LONG NOT NULL, `Version` CHAR(72), `Language` CHAR(20), `Attributes` INT, `Sequence` LONG NOT NULL PRIMARY KEY `File`)
INSERT INTO `File` (`File`, `Component_`, `FileName`, `FileSize`, `Sequence`) VALUES ('_FILE_0', '_COMPONENT_0', 'file1.txt', 0, 1)
INSERT INTO `File` (`File`, `Component_`, `FileName`, `FileSize`, `Sequence`) VALUES ('_FILE_1', '_COMPONENT_1', 'file2.pdf', 0, 2)
",
    );

    let mut verifiaction_command = Command::new("msiinfo");
    let command =
        verifiaction_command.args(["export", "-s", output_path, "Shortcut"]);
    command.assert().stdout( "\
CREATE TABLE `Shortcut` (`Shortcut` CHAR(72) NOT NULL, `Directory_` CHAR(72) NOT NULL, `Name` CHAR(128) NOT NULL LOCALIZABLE, `Component_` CHAR(72) NOT NULL, `Target` CHAR(72) NOT NULL, `Arguments` CHAR(255), `Description` CHAR(255) LOCALIZABLE, `Hotkey` INT, `Icon_` CHAR(72), `IconIndex` INT, `ShowCmd` INT, `WkDir` CHAR(72), `DisplayResourceDLL` CHAR(255), `DisplayResourceId` INT, `DescriptionResourceDLL` CHAR(255), `DescriptionResourceId` INT PRIMARY KEY `Shortcut`)
INSERT INTO `Shortcut` (`Shortcut`, `Directory_`, `Name`, `Component_`, `Target`) VALUES ('_SHORTCUT_0', 'DesktopFolder', 'files_shortcut', '_COMPONENT_2', '[InstallDir]/file1.txt')
",
    );

    let mut verifiaction_command = Command::new("msiinfo");
    let command = verifiaction_command.args([
        "export",
        "-s",
        output_path,
        "ServiceInstall",
    ]);
    command.assert().stdout(
        "\
CREATE TABLE `ServiceInstall` (`ServiceInstall` CHAR(72) NOT NULL, `Name` CHAR(255) NOT NULL, `DisplayName` CHAR(255) LOCALIZABLE, `ServiceType` LONG NOT NULL, `StartType` LONG NOT NULL, `ErrorControl` LONG NOT NULL, `LoadOrderGroup` CHAR(255), `Dependencies` CHAR(255), `StartName` CHAR(255), `Password` CHAR(255), `Arguments` CHAR(255), `Component_` CHAR(72) NOT NULL, `Description` CHAR(255) LOCALIZABLE PRIMARY KEY `ServiceInstall`)
INSERT INTO `ServiceInstall` (`ServiceInstall`, `Name`, `ServiceType`, `StartType`, `ErrorControl`, `Component_`) VALUES ('_SERVICEINST_0', 'MyService', 16, 2, 1, '_COMPONENT_2')
",
    );

    let mut verifiaction_command = Command::new("msiinfo");
    let command = verifiaction_command.args([
        "export",
        "-s",
        output_path,
        "ServiceControl",
    ]);
    command.assert().stdout(
        "\
CREATE TABLE `ServiceControl` (`ServiceControl` CHAR(72) NOT NULL, `Name` CHAR(255) NOT NULL, `Event` INT NOT NULL, `Arguments` CHAR(255), `Wait` INT, `Component_` CHAR(72) NOT NULL PRIMARY KEY `ServiceControl`)
INSERT INTO `ServiceControl` (`ServiceControl`, `Name`, `Event`, `Wait`, `Component_`) VALUES ('_SERVICECTRL_0', 'MyService', 321, 1, '_COMPONENT_2')
",
    );

    // TODO: Check permissions table
    // TODO: Check feature table
    // TODO: Check component_feature table
    // TODO: Check component table
    // TODO: Check MsiFileHash table
}
