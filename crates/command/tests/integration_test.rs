use assert_cmd::Command;
use assert_fs::TempDir;
use assert_fs::prelude::PathChild;

macro_rules! test_command {
    ($command:expr, $args:expr, $output:expr) => {
        let mut verification_command = Command::new($command);
        let command = verification_command.args($args);
        command.assert().stdout($output);
    };
}

#[test]
fn test_config_output() {
    const CONFIG_PATH: &str = "./tests/resources/example.ron";
    let temp_dir = TempDir::new().unwrap();
    let tmp_output = temp_dir.child("output.msi");
    let output_path = &tmp_output.path().to_string_lossy();
    macro_rules! test_table_output {
        ($table:expr, $output:expr) => {
            test_command!(
                "msiinfo",
                ["export", "-s", output_path, $table],
                $output
            )
        };
    }

    let mut base_command =
        Command::cargo_bin(assert_cmd::crate_name!()).unwrap();
    let command = base_command.args([
        "build",
        "--relative-to",
        "config",
        CONFIG_PATH,
        output_path,
    ]);

    let assert = command.assert();
    assert.success();

    test_command!(
        "msiinfo",
        ["streams", output_path],
        "\
_CABINET_0
Icon._ICON_0
\x05SummaryInformation
"
    );

    test_table_output!(
        "Directory",
        "\
CREATE TABLE `Directory` (`Directory` CHAR(72) NOT NULL, `Directory_Parent` CHAR(72), `DefaultDir` CHAR(255) NOT NULL LOCALIZABLE PRIMARY KEY `Directory`)
INSERT INTO `Directory` (`Directory`, `Directory_Parent`, `DefaultDir`) VALUES ('DesktopFolder', 'TARGETDIR', '.')
INSERT INTO `Directory` (`Directory`, `Directory_Parent`, `DefaultDir`) VALUES ('ProgramFilesFolder', 'TARGETDIR', '.')
INSERT INTO `Directory` (`Directory`, `DefaultDir`) VALUES ('TARGETDIR', 'SourceDir')
INSERT INTO `Directory` (`Directory`, `Directory_Parent`, `DefaultDir`) VALUES ('_DIRECTORY_0', 'ProgramFilesFolder', 'Manny')
INSERT INTO `Directory` (`Directory`, `Directory_Parent`, `DefaultDir`) VALUES ('_DIRECTORY_1', '_DIRECTORY_0', 'PING')
INSERT INTO `Directory` (`Directory`, `Directory_Parent`, `DefaultDir`) VALUES ('_DIRECTORY_2', '_DIRECTORY_1', 'bin')
INSERT INTO `Directory` (`Directory`, `Directory_Parent`, `DefaultDir`) VALUES ('_DIRECTORY_3', '_DIRECTORY_1', 'data')
INSERT INTO `Directory` (`Directory`, `Directory_Parent`, `DefaultDir`) VALUES ('_DIRECTORY_4', '_DIRECTORY_3', 'config')
"
    );

    test_table_output!(
        "File",
        "\
CREATE TABLE `File` (`File` CHAR(72) NOT NULL, `Component_` CHAR(72) NOT NULL, `FileName` CHAR(255) NOT NULL LOCALIZABLE, `FileSize` LONG NOT NULL, `Version` CHAR(72), `Language` CHAR(20), `Attributes` INT, `Sequence` LONG NOT NULL PRIMARY KEY `File`)
INSERT INTO `File` (`File`, `Component_`, `FileName`, `FileSize`, `Sequence`) VALUES ('_FILE_0', '_COMPONENT_0', 'ping.exe', 5464691, 1)
INSERT INTO `File` (`File`, `Component_`, `FileName`, `FileSize`, `Sequence`) VALUES ('_FILE_1', '_COMPONENT_1', 'options.conf', 14, 2)
INSERT INTO `File` (`File`, `Component_`, `FileName`, `FileSize`, `Sequence`) VALUES ('_FILE_2', '_COMPONENT_2', 'icon.ico', 67646, 3)
"
    );

    test_table_output!(
        "Shortcut",
        "\
CREATE TABLE `Shortcut` (`Shortcut` CHAR(72) NOT NULL, `Directory_` CHAR(72) NOT NULL, `Name` CHAR(128) NOT NULL LOCALIZABLE, `Component_` CHAR(72) NOT NULL, `Target` CHAR(72) NOT NULL, `Arguments` CHAR(255), `Description` CHAR(255) LOCALIZABLE, `Hotkey` INT, `Icon_` CHAR(72), `IconIndex` INT, `ShowCmd` INT, `WkDir` CHAR(72), `DisplayResourceDLL` CHAR(255), `DisplayResourceId` INT, `DescriptionResourceDLL` CHAR(255), `DescriptionResourceId` INT PRIMARY KEY `Shortcut`)
INSERT INTO `Shortcut` (`Shortcut`, `Directory_`, `Name`, `Component_`, `Target`, `Icon_`, `IconIndex`) VALUES ('_SHORTCUT_0', 'DesktopFolder', '_SHORTCUT_0', '_COMPONENT_3', '[InstallDir]/bin/ping.exe', '_ICON_0', 0)
"
    );

    test_table_output!(
        "ServiceInstall",
        "\
CREATE TABLE `ServiceInstall` (`ServiceInstall` CHAR(72) NOT NULL, `Name` CHAR(255) NOT NULL, `DisplayName` CHAR(255) LOCALIZABLE, `ServiceType` LONG NOT NULL, `StartType` LONG NOT NULL, `ErrorControl` LONG NOT NULL, `LoadOrderGroup` CHAR(255), `Dependencies` CHAR(255), `StartName` CHAR(255), `Password` CHAR(255), `Arguments` CHAR(255), `Component_` CHAR(72) NOT NULL, `Description` CHAR(255) LOCALIZABLE PRIMARY KEY `ServiceInstall`)
INSERT INTO `ServiceInstall` (`ServiceInstall`, `Name`, `ServiceType`, `StartType`, `ErrorControl`, `Component_`) VALUES ('_SERVICEINST_0', 'PingService', 16, 2, 1, '_COMPONENT_3')
"
    );

    test_table_output!(
        "ServiceControl",
        "\
CREATE TABLE `ServiceControl` (`ServiceControl` CHAR(72) NOT NULL, `Name` CHAR(255) NOT NULL, `Event` INT NOT NULL, `Arguments` CHAR(255), `Wait` INT, `Component_` CHAR(72) NOT NULL PRIMARY KEY `ServiceControl`)
INSERT INTO `ServiceControl` (`ServiceControl`, `Name`, `Event`, `Wait`, `Component_`) VALUES ('_SERVICECTRL_0', 'PingService', 321, 1, '_COMPONENT_3')
"
    );

    test_table_output!(
        "LockPermissions",
        "\
CREATE TABLE `LockPermissions` (`LockObject` CHAR(72) NOT NULL, `Table` CHAR(255) NOT NULL, `Domain` CHAR(255), `User` CHAR(255) NOT NULL, `Permission` LONG PRIMARY KEY `LockObject`)
INSERT INTO `LockPermissions` (`LockObject`, `Table`, `User`, `Permission`) VALUES ('_FILE_1', 'File', 'Everyone', 0)
"
    );

    test_table_output!(
        "Feature",
        "\
CREATE TABLE `Feature` (`Feature` CHAR(38) NOT NULL, `Feature_Parent` CHAR(38), `Title` CHAR(64) LOCALIZABLE, `Description` CHAR(255) LOCALIZABLE, `Display` INT, `Level` INT NOT NULL, `Directory_` CHAR(72), `Attributes` INT NOT NULL PRIMARY KEY `Feature`)
INSERT INTO `Feature` (`Feature`, `Title`, `Display`, `Level`, `Attributes`) VALUES ('DEFAULT_FEATURE', 'Default Feature', 2, 1, 0)
"
    );

    test_table_output!(
        "FeatureComponents",
        "\
CREATE TABLE `FeatureComponents` (`Feature_` CHAR(38) NOT NULL, `Component_` CHAR(72) NOT NULL PRIMARY KEY `Feature_`, `Component_`)
INSERT INTO `FeatureComponents` (`Feature_`, `Component_`) VALUES ('DEFAULT_FEATURE', '_COMPONENT_0')
INSERT INTO `FeatureComponents` (`Feature_`, `Component_`) VALUES ('DEFAULT_FEATURE', '_COMPONENT_1')
INSERT INTO `FeatureComponents` (`Feature_`, `Component_`) VALUES ('DEFAULT_FEATURE', '_COMPONENT_2')
INSERT INTO `FeatureComponents` (`Feature_`, `Component_`) VALUES ('DEFAULT_FEATURE', '_COMPONENT_3')
"
    );

    test_table_output!(
        "Component",
        "\
CREATE TABLE `Component` (`Component` CHAR(72) NOT NULL, `ComponentId` CHAR(38), `Directory_` CHAR(72) NOT NULL, `Attributes` INT NOT NULL, `Condition` CHAR(255), `KeyPath` CHAR(72) PRIMARY KEY `Component`)
INSERT INTO `Component` (`Component`, `Directory_`, `Attributes`, `KeyPath`) VALUES ('_COMPONENT_0', '_DIRECTORY_2', 0, '_FILE_0')
INSERT INTO `Component` (`Component`, `Directory_`, `Attributes`, `KeyPath`) VALUES ('_COMPONENT_1', '_DIRECTORY_4', 0, '_FILE_1')
INSERT INTO `Component` (`Component`, `Directory_`, `Attributes`, `KeyPath`) VALUES ('_COMPONENT_2', '_DIRECTORY_3', 0, '_FILE_2')
"
    );

    test_table_output!(
        "MsiFileHash",
        "\
CREATE TABLE `MsiFileHash` (`File_` CHAR(72) NOT NULL, `Options` INT NOT NULL, `HashPart1` LONG NOT NULL, `HashPart2` LONG NOT NULL, `HashPart3` LONG NOT NULL, `HashPart4` LONG NOT NULL PRIMARY KEY `File_`)
INSERT INTO `MsiFileHash` (`File_`, `Options`, `HashPart1`, `HashPart2`, `HashPart3`, `HashPart4`) VALUES ('_FILE_0', 0, -486793845, -1978109702, -1585535730, 508107097)
INSERT INTO `MsiFileHash` (`File_`, `Options`, `HashPart1`, `HashPart2`, `HashPart3`, `HashPart4`) VALUES ('_FILE_1', 0, 68558604, 539115277, 2048134204, -872244547)
INSERT INTO `MsiFileHash` (`File_`, `Options`, `HashPart1`, `HashPart2`, `HashPart3`, `HashPart4`) VALUES ('_FILE_2', 0, -139036463, 1225729016, -1423872358, 501321119)
"
    );

    test_table_output!(
        "Icon",
        "\
CREATE TABLE `Icon` (`Name` CHAR(72) NOT NULL, `Data` OBJECT NOT NULL PRIMARY KEY `Name`)
INSERT INTO `Icon` (`Name`, `Data`) VALUES ('_ICON_0', '')
"
    );
}
