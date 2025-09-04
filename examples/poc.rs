#![allow(unused_variables)]
// This Proof of concept is just trying to match an MSI made by a custom script
// I made earlier.
//
// Information was gathered using the `msitools` package that we are trying to
// replace :p
//
// This may look like an extremely verbose mess but I wanted to outline exactly
// what is going to be needed to get a working MSI.
//
// ## Tables
//
// Tables present in MSI:
// - ActionText
// - AdminExecuteSequence
// - AdminUISequence
// - AdvtExecuteSequence
// - AppSearch
// - Binary
// - Component
// - Condition
// - CreateFolder
// - CustomAction
// - Directory
// - Environment
// - Error
// - Feature
// - FeatureComponents
// - File
// - Icon
// - IniFile
// - InstallExecuteSequence
// - InstallUISequence
// - LaunchCondition
// - Media
// - MsiFileHash
// - Property
// - RegLocator
// - Registry
// - RemoveFile
// - RemoveIniFile
// - ServiceControl
// - ServiceInstall
// - Shortcut
// - Signature
// - Upgrade
// - _ForceCodepage
// - _SummaryInformation
//
// Tables with at least 1 row according to MSI Viewer (Microsoft Store App):
//
// - AdminExecuteSequence
// - AdminUISequence
// - AdvtExecuteSequence
// - Component
// - Directory
// - Feature
// - FeatureComponents
// - File
// - Icon
// - InstallExecuteSequence
// - InstallUISequence
// - Media
// - MsiFileHash
// - Property
// - Registry
// - RemoveFile <- Skipping for now as I think this is unnecessary.
// - ServiceControl
// - ServiceInstall
// - Shortcut

use std::fs::File;
use std::io::Cursor;
use std::io::Write;
use std::path::PathBuf;
use std::time::Duration;
use std::time::UNIX_EPOCH;

use anyhow::Context;
use anyhow::Result;
use msi::Column;
use msi::Insert;
use msi::Value;

macro_rules! to_values {
    ($($x:expr),*) => {
        vec![$(Value::from($x)),*]
    };
}

use msi::Language;

type MsiPackage = msi::Package<Cursor<Vec<u8>>>;

fn main() {
    let mut msi = msi::Package::create(
        msi::PackageType::Installer,
        Cursor::new(Vec::new()),
    )
    .unwrap();
    set_summary_information(&mut msi);
    set_admin_execute_sequence(&mut msi);
    set_admin_ui_sequence(&mut msi);
    set_advt_execute_sequence(&mut msi);
    set_component(&mut msi);
    set_directory(&mut msi);
    set_feature(&mut msi);
    set_feature_components(&mut msi);
    set_file(&mut msi);
    set_icon(&mut msi);
    set_install_execute_sequence(&mut msi);
    set_install_ui_sequence(&mut msi);
    set_media(&mut msi);
    set_msi_file_hash(&mut msi);
    set_property(&mut msi);
    set_registry(&mut msi);
    set_service_control(&mut msi);
    set_service_install(&mut msi);
    set_shortcut(&mut msi);
    // TODO: Write the streams to the MSI
    write(msi, &PathBuf::from("test.msi")).unwrap();
}

fn write(msi: MsiPackage, output_path: &PathBuf) -> Result<()> {
    let cursor = msi.into_inner().unwrap();
    let mut file = File::create(output_path)
        .context(format!("Open output path {output_path:?} for writing"))?;

    file.write_all(cursor.get_ref())
        .context(format!("Write MSI data to location {output_path:?}"))?;
    Ok(())
}

// ### Summary Information
//

// Title: Installation Database
// Subject: PING
// Author: Manny
// Keywords: Installer; 0.1.0
// Comments: Summary of PING application
// Template: x64;1033
// Revision number (UUID): {380BD173-8D6F-4FCE-ABA7-4368EC40E5DE}
// Created: Thu Aug 21 13:33:58 2025
// Last saved: Thu Aug 21 13:33:58 2025
// Version: 200 (c8)
// Source: 2 (2)
// Application: msitools 0.106
// Security: 2 (2)
//
fn set_summary_information(msi: &mut MsiPackage) {
    let sum = msi.summary_info_mut();
    sum.set_title("Installation Database");
    sum.set_subject("PING");
    sum.set_author("Manny");
    sum.set_comments("Summary of PING application");
    // Sets the architecture string in the `template` property.
    sum.set_arch("x64");
    // Sets the language string in the `template` property.
    sum.set_languages(&[Language::from_code(1033)]);
    // Performs some aspects of the `Revision number` property.
    sum.set_uuid("{380BD173-8D6F-4FCE-ABA7-4368EC40E5DE}".parse().unwrap());
    let timestamp = 1751484692;
    let dur = Duration::from_secs(timestamp);
    sum.set_creation_time(UNIX_EPOCH.checked_add(dur).unwrap());
    sum.set_creating_application("msitools 0.106");
}

// CREATE TABLE `AdminExecuteSequence` (`Action` CHAR(72) NOT NULL, `Condition`
// CHAR(255), `Sequence` INT PRIMARY KEY `Action`) INSERT INTO
// `AdminExecuteSequence` (`Action`, `Sequence`) VALUES ('CostInitialize', 800)
// INSERT INTO `AdminExecuteSequence` (`Action`, `Sequence`) VALUES ('FileCost',
// 900) INSERT INTO `AdminExecuteSequence` (`Action`, `Sequence`) VALUES
// ('CostFinalize', 1000) INSERT INTO `AdminExecuteSequence` (`Action`,
// `Sequence`) VALUES ('InstallValidate', 1400) INSERT INTO
// `AdminExecuteSequence` (`Action`, `Sequence`) VALUES ('InstallInitialize',
// 1500) INSERT INTO `AdminExecuteSequence` (`Action`, `Sequence`) VALUES
// ('InstallFiles', 4000) INSERT INTO `AdminExecuteSequence` (`Action`,
// `Sequence`) VALUES ('InstallFinalize', 6600) INSERT INTO
// `AdminExecuteSequence` (`Action`, `Sequence`) VALUES ('InstallAdminPackage',
// 3900)
fn set_admin_execute_sequence(msi: &mut MsiPackage) {
    let table_name = "AdminExecuteSequence";
    let columns = vec![
        Column::build("Action").primary_key().string(72),
        Column::build("Condition").nullable().string(255),
        Column::build("Sequence").nullable().int16(),
    ];
    msi.create_table(table_name, columns).unwrap();
    let query = Insert::into(table_name).rows(vec![
        to_values!["CostInitialize", Value::Null, 800],
        to_values!["FileCost", Value::Null, 900],
        to_values!["CostFinalize", Value::Null, 1000],
        to_values!["InstallValidate", Value::Null, 1400],
        to_values!["InstallInitialize", Value::Null, 1500],
        to_values!["InstallFiles", Value::Null, 4000],
        to_values!["InstallFinalize", Value::Null, 6600],
        to_values!["InstallAdminPackage", Value::Null, 3900],
    ]);
    msi.insert_rows(query).unwrap()
}

// CREATE TABLE `AdminUISequence` (`Action` CHAR(72) NOT NULL, `Condition`
// CHAR(255), `Sequence` INT PRIMARY KEY `Action`) INSERT INTO `AdminUISequence`
// (`Action`, `Sequence`) VALUES ('CostInitialize', 800) INSERT INTO
// `AdminUISequence` (`Action`, `Sequence`) VALUES ('FileCost', 900) INSERT INTO
// `AdminUISequence` (`Action`, `Sequence`) VALUES ('CostFinalize', 1000) INSERT
// INTO `AdminUISequence` (`Action`, `Sequence`) VALUES ('ExecuteAction', 1300)

fn set_admin_ui_sequence(msi: &mut MsiPackage) {
    let table_name = "AdminUISequence";
    let columns = vec![
        Column::build("Action").primary_key().string(72),
        Column::build("Condition").nullable().string(255),
        Column::build("Sequence").nullable().int16(),
    ];
    msi.create_table(table_name, columns).unwrap();
    let query = Insert::into(table_name).rows(vec![
        vec![Value::from("CostInitialize"), Value::Null, Value::from(800)],
        vec![Value::from("FileCost"), Value::Null, Value::from(900)],
        vec![Value::from("CostFinalize"), Value::Null, Value::from(1000)],
        vec![Value::from("ExecuteAction"), Value::Null, Value::from(1300)],
    ]);
    msi.insert_rows(query).unwrap()
}

// CREATE TABLE `AdvtExecuteSequence` (`Action` CHAR(72) NOT NULL, `Condition`
// CHAR(255), `Sequence` INT PRIMARY KEY `Action`) INSERT INTO
// `AdvtExecuteSequence` (`Action`, `Sequence`) VALUES ('CostInitialize', 800)
// INSERT INTO `AdvtExecuteSequence` (`Action`, `Sequence`) VALUES
// ('CostFinalize', 1000) INSERT INTO `AdvtExecuteSequence` (`Action`,
// `Sequence`) VALUES ('InstallValidate', 1400) INSERT INTO
// `AdvtExecuteSequence` (`Action`, `Sequence`) VALUES ('InstallInitialize',
// 1500) INSERT INTO `AdvtExecuteSequence` (`Action`, `Sequence`) VALUES
// ('CreateShortcuts', 4500) INSERT INTO `AdvtExecuteSequence` (`Action`,
// `Sequence`) VALUES ('PublishFeatures', 6300) INSERT INTO
// `AdvtExecuteSequence` (`Action`, `Sequence`) VALUES ('PublishProduct', 6400)
// INSERT INTO `AdvtExecuteSequence` (`Action`, `Sequence`) VALUES
// ('InstallFinalize', 6600)
fn set_advt_execute_sequence(msi: &mut MsiPackage) {
    let table_name = "AdvtExecuteSequence";
    let columns = vec![
        Column::build("Action").primary_key().string(72),
        Column::build("Condition").nullable().string(255),
        Column::build("Sequence").nullable().int16(),
    ];
    msi.create_table(table_name, columns).unwrap();
    let query = Insert::into(table_name).rows(vec![
        to_values!("CostInitialize", Value::Null, 800),
        to_values!("CostFinalize", Value::Null, 1000),
        to_values!("InstallValidate", Value::Null, 1400),
        to_values!("InstallInitialize", Value::Null, 1500),
        to_values!("CreateShortcuts", Value::Null, 4500),
        to_values!("PublishFeatures", Value::Null, 6300),
        to_values!("PublishProduct", Value::Null, 6400),
        to_values!("InstallFinalize", Value::Null, 6600),
    ]);
    msi.insert_rows(query).unwrap()
}

// CREATE TABLE `Component` (`Component` CHAR(72) NOT NULL, `ComponentId`
// CHAR(38), `Directory_` CHAR(72) NOT NULL, `Attributes` INT NOT NULL,
// `Condition` CHAR(255), `KeyPath` CHAR(72) PRIMARY KEY `Component`)
// INSERT INTO `Component` (`Component`, `ComponentId`, `Directory_`,
// `Attributes`, `KeyPath`) VALUES ('6',
// '{A8A7366E-5D64-54C1-9C59-03E36DB118DD}', '3', 256, '7') INSERT INTO
// `Component` (`Component`, `ComponentId`, `Directory_`, `Attributes`,
// `KeyPath`) VALUES ('0', '{562E2D89-E877-5B62-AA1B-D16F319A1EA7}',
// 'DesktopFolder', 260, 'reg8B8E36E486B293215DD585990A46A8CE') INSERT INTO
// `Component` (`Component`, `ComponentId`, `Directory_`, `Attributes`,
// `KeyPath`) VALUES ('10', '{79D06F27-8B3F-5731-A8AF-D4B734DE70CC}', '5', 256,
// '11') INSERT INTO `Component` (`Component`, `ComponentId`, `Directory_`,
// `Attributes`, `KeyPath`) VALUES ('13',
// '{493285EE-0C0C-5551-9B96-9F6AA0D115E5}', '4', 256, '14')
fn set_component(msi: &mut MsiPackage) {
    let table_name = "Component";
    let columns = vec![
        Column::build("Component").primary_key().string(72),
        Column::build("ComponentId").nullable().string(38),
        Column::build("Directory_").string(72),
        Column::build("Attributes").int16(),
        Column::build("Condition").nullable().string(255),
        Column::build("KeyPath").nullable().string(72),
    ];
    msi.create_table(table_name, columns).unwrap();
    let query = Insert::into(table_name).rows(vec![
        to_values!(
            "6",
            "{A8A7366E-5D64-54C1-9C59-03E36DB118DD}",
            "3",
            256,
            Value::Null,
            "7"
        ),
        to_values!(
            "0",
            "{562E2D89-E877-5B62-AA1B-D16F319A1EA7}",
            "DesktopFolder",
            260,
            Value::Null,
            "reg8B8E36E486B293215DD585990A46A8CE"
        ),
        to_values!(
            "10",
            "{79D06F27-8B3F-5731-A8AF-D4B734DE70CC}",
            "5",
            256,
            Value::Null,
            "11"
        ),
        to_values!(
            "13",
            "{493285EE-0C0C-5551-9B96-9F6AA0D115E5}",
            "4",
            256,
            Value::Null,
            "14"
        ),
    ]);
    msi.insert_rows(query).unwrap()
}

// CREATE TABLE `Directory` (`Directory` CHAR(72) NOT NULL, `Directory_Parent`
// CHAR(72), `DefaultDir` CHAR(255) NOT NULL LOCALIZABLE PRIMARY KEY
// `Directory`) INSERT INTO `Directory` (`Directory`, `Directory_Parent`,
// `DefaultDir`) VALUES ('DesktopFolder', 'TARGETDIR', 'Desktop') INSERT INTO
// `Directory` (`Directory`, `Directory_Parent`, `DefaultDir`) VALUES
// ('INSTALLDIR', 'manny', 'PING') INSERT INTO `Directory` (`Directory`,
// `DefaultDir`) VALUES ('TARGETDIR', 'SourceDir') INSERT INTO `Directory`
// (`Directory`, `Directory_Parent`, `DefaultDir`) VALUES ('3', 'INSTALLDIR',
// 'bin') INSERT INTO `Directory` (`Directory`, `Directory_Parent`,
// `DefaultDir`) VALUES ('5', '4', 'config') INSERT INTO `Directory`
// (`Directory`, `Directory_Parent`, `DefaultDir`) VALUES ('4', 'INSTALLDIR',
// 'data') INSERT INTO `Directory` (`Directory`, `Directory_Parent`,
// `DefaultDir`) VALUES ('manny', 'ProgramFiles64Folder', 'manny') INSERT INTO
// `Directory` (`Directory`, `Directory_Parent`, `DefaultDir`) VALUES
// ('ProgramFiles64Folder', 'TARGETDIR', 'PFiles')
fn set_directory(msi: &mut MsiPackage) {
    let table_name = "Directory";
    let columns = vec![
        Column::build("Directory").primary_key().string(72),
        Column::build("Directory_Parent").nullable().string(72),
        Column::build("DefaultDir").localizable().string(255),
    ];
    msi.create_table(table_name, columns).unwrap();
    let query = Insert::into(table_name).rows(vec![
        to_values!("DesktopFolder", "TARGETDIR", "Desktop"),
        to_values!("INSTALLDIR", "manny", "PING"),
        to_values!("TARGETDIR", Value::Null, "SourceDir"),
        to_values!("3", "INSTALLDIR", "bin"),
        to_values!("5", "4", "config"),
        to_values!("4", "INSTALLDIR", "data"),
        to_values!("manny", "ProgramFiles64Folder", "manny"),
        to_values!("ProgramFiles64Folder", "TARGETDIR", "PFiles"),
    ]);
    msi.insert_rows(query).unwrap()
}

// NOTE: In my ignorance when first making the original script I made every
// component it's own feature.
//
// CREATE TABLE `Feature` (`Feature` CHAR(38) NOT NULL, `Feature_Parent`
// CHAR(38), `Title` CHAR(64) LOCALIZABLE, `Description` CHAR(255) LOCALIZABLE,
// `Display` INT, `Level` INT NOT NULL, `Directory_` CHAR(72), `Attributes` INT
// NOT NULL PRIMARY KEY `Feature`) INSERT INTO `Feature` (`Feature`, `Display`,
// `Level`, `Attributes`) VALUES ('2', 2, 1, 0) INSERT INTO `Feature`
// (`Feature`, `Display`, `Level`, `Attributes`) VALUES ('8', 2, 1, 0)
// INSERT INTO `Feature` (`Feature`, `Display`, `Level`, `Attributes`) VALUES
// ('12', 2, 1, 0) INSERT INTO `Feature` (`Feature`, `Display`, `Level`,
// `Attributes`) VALUES ('15', 2, 1, 0)
fn set_feature(msi: &mut MsiPackage) {
    let table_name = "Feature";
    let columns = vec![
        Column::build("Feature").primary_key().string(38),
        Column::build("Feature_Parent").nullable().string(38),
        Column::build("Title").nullable().localizable().string(64),
        Column::build("Description").nullable().localizable().string(255),
        Column::build("Display").nullable().int16(),
        Column::build("Level").int16(),
        Column::build("Directory_").nullable().string(72),
        Column::build("Attributes").int16(),
    ];
    msi.create_table(table_name, columns).unwrap();
    let query = Insert::into(table_name).rows(vec![
        to_values!(
            "2",
            Value::Null,
            Value::Null,
            Value::Null,
            2,
            1,
            Value::Null,
            0
        ),
        to_values!(
            "8",
            Value::Null,
            Value::Null,
            Value::Null,
            2,
            1,
            Value::Null,
            0
        ),
        to_values!(
            "12",
            Value::Null,
            Value::Null,
            Value::Null,
            2,
            1,
            Value::Null,
            0
        ),
        to_values!(
            "15",
            Value::Null,
            Value::Null,
            Value::Null,
            2,
            1,
            Value::Null,
            0
        ),
    ]);
    msi.insert_rows(query).unwrap()
}

// CREATE TABLE `FeatureComponents` (`Feature_` CHAR(38) NOT NULL, `Component_`
// CHAR(72) NOT NULL PRIMARY KEY `Feature_`, `Component_`) INSERT INTO
// `FeatureComponents` (`Feature_`, `Component_`) VALUES ('2', '0') INSERT INTO
// `FeatureComponents` (`Feature_`, `Component_`) VALUES ('8', '6') INSERT INTO
// `FeatureComponents` (`Feature_`, `Component_`) VALUES ('12', '10')
// INSERT INTO `FeatureComponents` (`Feature_`, `Component_`) VALUES ('15',
// '13')
fn set_feature_components(msi: &mut MsiPackage) {
    let table_name = "FeatureComponents";
    let columns = vec![
        Column::build("Feature_").primary_key().string(38),
        Column::build("Component_").primary_key().string(72),
    ];
    msi.create_table(table_name, columns).unwrap();
    let query = Insert::into(table_name).rows(vec![
        to_values!("2", "0"),
        to_values!("8", "6"),
        to_values!("12", "10"),
        to_values!("15", "13"),
    ]);
    msi.insert_rows(query).unwrap()
}

// CREATE TABLE `File` (`File` CHAR(72) NOT NULL, `Component_` CHAR(72) NOT
// NULL, `FileName` CHAR(255) NOT NULL LOCALIZABLE, `FileSize` LONG NOT NULL,
// `Version` CHAR(72), `Language` CHAR(20), `Attributes` INT, `Sequence` LONG
// NOT NULL PRIMARY KEY `File`) INSERT INTO `File` (`File`, `Component_`,
// `FileName`, `FileSize`, `Attributes`, `Sequence`) VALUES ('7', '6',
// 'ping.exe', 5464691, 512, 1) INSERT INTO `File` (`File`, `Component_`,
// `FileName`, `FileSize`, `Attributes`, `Sequence`) VALUES ('11', '10',
// 'options.conf', 14, 512, 2) INSERT INTO `File` (`File`, `Component_`,
// `FileName`, `FileSize`, `Attributes`, `Sequence`) VALUES ('14', '13',
// 'icon.ico', 67646, 512, 3)
fn set_file(msi: &mut MsiPackage) {
    let table_name = "File";
    let columns = vec![
        Column::build("File").primary_key().string(72),
        Column::build("Component_").string(72),
        Column::build("FileName").localizable().string(255),
        Column::build("FileSize").int32(),
        Column::build("Version").nullable().string(72),
        Column::build("Language").nullable().string(20),
        Column::build("Attributes").nullable().int16(),
        Column::build("Sequence").int32(),
    ];
    msi.create_table(table_name, columns).unwrap();
    let query = Insert::into(table_name).rows(vec![
        to_values![
            "7",
            "6",
            "ping.exe",
            5464691,
            Value::Null,
            Value::Null,
            512,
            1
        ],
        to_values![
            "11",
            "10",
            "options.conf",
            14,
            Value::Null,
            Value::Null,
            512,
            2
        ],
        to_values![
            "14",
            "13",
            "icon.ico",
            67646,
            Value::Null,
            Value::Null,
            512,
            3
        ],
    ]);
    msi.insert_rows(query).unwrap()
}

// CREATE TABLE `Icon` (`Name` CHAR(72) NOT NULL, `Data` OBJECT NOT NULL PRIMARY
// KEY `Name`) INSERT INTO `Icon` (`Name`, `Data`) VALUES ('1', '')
fn set_icon(msi: &mut MsiPackage) {
    let table_name = "Icon";
    let columns = vec![
        Column::build("Name").primary_key().string(72),
        Column::build("Data").string(255),
    ];
    msi.create_table(table_name, columns).unwrap();
    let query = Insert::into(table_name).rows(vec![to_values!["1", ""]]);
    msi.insert_rows(query).unwrap()
}

// CREATE TABLE `InstallExecuteSequence` (`Action` CHAR(72) NOT NULL,
// `Condition` CHAR(255), `Sequence` INT PRIMARY KEY `Action`) INSERT INTO
// `InstallExecuteSequence` (`Action`, `Sequence`) VALUES ('CostInitialize',
// 800) INSERT INTO `InstallExecuteSequence` (`Action`, `Sequence`) VALUES
// ('FileCost', 900) INSERT INTO `InstallExecuteSequence` (`Action`, `Sequence`)
// VALUES ('CostFinalize', 1000) INSERT INTO `InstallExecuteSequence` (`Action`,
// `Sequence`) VALUES ('ValidateProductID', 700) INSERT INTO
// `InstallExecuteSequence` (`Action`, `Sequence`) VALUES ('InstallValidate',
// 1400) INSERT INTO `InstallExecuteSequence` (`Action`, `Sequence`) VALUES
// ('InstallInitialize', 1500) INSERT INTO `InstallExecuteSequence` (`Action`,
// `Sequence`) VALUES ('ProcessComponents', 1600) INSERT INTO
// `InstallExecuteSequence` (`Action`, `Sequence`) VALUES ('UnpublishFeatures',
// 1800) INSERT INTO `InstallExecuteSequence` (`Action`, `Condition`,
// `Sequence`) VALUES ('StopServices', 'VersionNT', 1900) INSERT INTO
// `InstallExecuteSequence` (`Action`, `Condition`, `Sequence`) VALUES
// ('DeleteServices', 'VersionNT', 2000) INSERT INTO `InstallExecuteSequence`
// (`Action`, `Sequence`) VALUES ('RemoveRegistryValues', 2600) INSERT INTO
// `InstallExecuteSequence` (`Action`, `Sequence`) VALUES ('RemoveShortcuts',
// 3200) INSERT INTO `InstallExecuteSequence` (`Action`, `Sequence`) VALUES
// ('RemoveFiles', 3500) INSERT INTO `InstallExecuteSequence` (`Action`,
// `Sequence`) VALUES ('InstallFiles', 4000) INSERT INTO
// `InstallExecuteSequence` (`Action`, `Sequence`) VALUES ('CreateShortcuts',
// 4500) INSERT INTO `InstallExecuteSequence` (`Action`, `Sequence`) VALUES
// ('WriteRegistryValues', 5000) INSERT INTO `InstallExecuteSequence` (`Action`,
// `Condition`, `Sequence`) VALUES ('InstallServices', 'VersionNT', 5800) INSERT
// INTO `InstallExecuteSequence` (`Action`, `Condition`, `Sequence`) VALUES
// ('StartServices', 'VersionNT', 5900) INSERT INTO `InstallExecuteSequence`
// (`Action`, `Sequence`) VALUES ('RegisterUser', 6000) INSERT INTO
// `InstallExecuteSequence` (`Action`, `Sequence`) VALUES ('RegisterProduct',
// 6100) INSERT INTO `InstallExecuteSequence` (`Action`, `Sequence`) VALUES
// ('PublishFeatures', 6300) INSERT INTO `InstallExecuteSequence` (`Action`,
// `Sequence`) VALUES ('PublishProduct', 6400) INSERT INTO
// `InstallExecuteSequence` (`Action`, `Sequence`) VALUES ('InstallFinalize',
// 6600)
fn set_install_execute_sequence(msi: &mut MsiPackage) {
    let table_name = "InstallExecuteSequence";
    let columns = vec![
        Column::build("Action").primary_key().string(72),
        Column::build("Condition").nullable().string(255),
        Column::build("Sequence").nullable().int16(),
    ];
    msi.create_table(table_name, columns).unwrap();
    let query = Insert::into(table_name).rows(vec![
        to_values!["CostInitialize", Value::Null, 800],
        to_values!["FileCost", Value::Null, 900],
        to_values!["CostFinalize", Value::Null, 1000],
        to_values!["ValidateProductID", Value::Null, 700],
        to_values!["InstallValidate", Value::Null, 1400],
        to_values!["InstallInitialize", Value::Null, 1500],
        to_values!["ProcessComponents", Value::Null, 1600],
        to_values!["UnpublishFeatures", Value::Null, 1800],
        to_values!["StopServices", "VersionNT", 1900],
        to_values!["DeleteServices", "VersionNT", 2000],
        to_values!["RemoveRegistryValues", Value::Null, 2600],
        to_values!["RemoveShortcuts", Value::Null, 3200],
        // NOTE: Commenting this out since RemoveFiles table doesn't exist
        // to_values!["RemoveFiles", Value::Null, 3500],
        to_values!["InstallFiles", Value::Null, 4000],
        to_values!["CreateShortcuts", Value::Null, 4500],
        to_values!["WriteRegistryValues", Value::Null, 5000],
        to_values!["InstallServices", "VersionNT", 5800],
        to_values!["StartServices", "VersionNT", 5900],
        to_values!["RegisterUser", Value::Null, 6000],
        to_values!["RegisterProduct", Value::Null, 6100],
        to_values!["PublishFeatures", Value::Null, 6300],
        to_values!["PublishProduct", Value::Null, 6400],
        to_values!["InstallFinalize", Value::Null, 6600],
    ]);
    msi.insert_rows(query).unwrap()
}

// CREATE TABLE `InstallUISequence` (`Action` CHAR(72) NOT NULL, `Condition`
// CHAR(255), `Sequence` INT PRIMARY KEY `Action`) INSERT INTO
// `InstallUISequence` (`Action`, `Sequence`) VALUES ('CostInitialize', 800)
// INSERT INTO `InstallUISequence` (`Action`, `Sequence`) VALUES ('FileCost',
// 900) INSERT INTO `InstallUISequence` (`Action`, `Sequence`) VALUES
// ('CostFinalize', 1000) INSERT INTO `InstallUISequence` (`Action`, `Sequence`)
// VALUES ('ExecuteAction', 1300) INSERT INTO `InstallUISequence` (`Action`,
// `Sequence`) VALUES ('ValidateProductID', 700)
fn set_install_ui_sequence(msi: &mut MsiPackage) {
    let table_name = "InstallUISequence";
    let columns = vec![
        Column::build("Action").primary_key().string(72),
        Column::build("Condition").nullable().string(255),
        Column::build("Sequence").nullable().int16(),
    ];
    msi.create_table(table_name, columns).unwrap();
    let query = Insert::into(table_name).rows(vec![
        to_values!["CostInitialize", Value::Null, 800],
        to_values!["FileCost", Value::Null, 900],
        to_values!["CostFinalize", Value::Null, 1000],
        to_values!["ExecuteAction", Value::Null, 1300],
        to_values!["ValidateProductID", Value::Null, 700],
    ]);
    msi.insert_rows(query).unwrap()
}

// CREATE TABLE `Media` (`DiskId` INT NOT NULL, `LastSequence` LONG NOT NULL,
// `DiskPrompt` CHAR(64) LOCALIZABLE, `Cabinet` CHAR(255), `VolumeLabel`
// CHAR(32), `Source` CHAR(72) PRIMARY KEY `DiskId`) INSERT INTO `Media`
// (`DiskId`, `LastSequence`, `DiskPrompt`, `Cabinet`) VALUES (1, 3, 'CD-ROM
// #1', '#PING.cab')
fn set_media(msi: &mut MsiPackage) {
    let table_name = "Media";
    let columns = vec![
        Column::build("DiskId").primary_key().int16(),
        Column::build("LastSequence").int32(),
        Column::build("DiskPrompt").nullable().localizable().string(64),
        Column::build("Cabinet").nullable().string(255),
        Column::build("VolumeLabel").nullable().string(32),
        Column::build("Source").nullable().string(72),
    ];
    msi.create_table(table_name, columns).unwrap();
    let query = Insert::into(table_name).rows(vec![to_values!(
        1,
        3,
        "CD-ROM #1",
        "#PING.cab",
        Value::Null,
        Value::Null
    )]);
    msi.insert_rows(query).unwrap()
}

// CREATE TABLE `MsiFileHash` (`File_` CHAR(72) NOT NULL, `Options` INT NOT
// NULL, `HashPart1` LONG NOT NULL, `HashPart2` LONG NOT NULL, `HashPart3` LONG
// NOT NULL, `HashPart4` LONG NOT NULL PRIMARY KEY `File_`) INSERT INTO
// `MsiFileHash` (`File_`, `Options`, `HashPart1`, `HashPart2`, `HashPart3`,
// `HashPart4`) VALUES ('7', 0, -486793845, -1978109702, -1585535730, 508107097)
// INSERT INTO `MsiFileHash` (`File_`, `Options`, `HashPart1`, `HashPart2`,
// `HashPart3`, `HashPart4`) VALUES ('11', 0, 68558604, 539115277, 2048134204,
// -872244547) INSERT INTO `MsiFileHash` (`File_`, `Options`, `HashPart1`,
// `HashPart2`, `HashPart3`, `HashPart4`) VALUES ('14', 0, -139036463,
// 1225729016, -1423872358, 501321119)
fn set_msi_file_hash(msi: &mut MsiPackage) {
    let table_name = "MsiFileHash";
    let columns = vec![
        Column::build("File_").primary_key().string(72),
        Column::build("Options").int16(),
        Column::build("HashPart1").int32(),
        Column::build("HashPart2").int32(),
        Column::build("HashPart3").int32(),
        Column::build("HashPart4").int32(),
    ];
    msi.create_table(table_name, columns).unwrap();
    let query = Insert::into(table_name).rows(vec![
        to_values!("7", 0, -486793845, -1978109702, -1585535730, 508107097),
        to_values!("11", 0, 68558604, 539115277, 2048134204, -872244547),
        to_values!("14", 0, -139036463, 1225729016, -1423872358, 501321119),
    ]);
    msi.insert_rows(query).unwrap()
}

// CREATE TABLE `Property` (`Property` CHAR(72) NOT NULL, `Value` CHAR(0) NOT
// NULL LOCALIZABLE PRIMARY KEY `Property`) INSERT INTO `Property` (`Property`,
// `Value`) VALUES ('DiskPrompt', 'PING Installation [1]') INSERT INTO
// `Property` (`Property`, `Value`) VALUES ('UpgradeCode', '{*}') INSERT INTO
// `Property` (`Property`, `Value`) VALUES ('ALLUSERS', '1') INSERT INTO
// `Property` (`Property`, `Value`) VALUES ('Manufacturer', 'Manny') INSERT INTO
// `Property` (`Property`, `Value`) VALUES ('ProductLanguage', '1033')
// INSERT INTO `Property` (`Property`, `Value`) VALUES ('ProductCode',
// '{328FDEA1-B4AF-4461-91A9-60F0B3C63DB5}') INSERT INTO `Property` (`Property`,
// `Value`) VALUES ('ProductName', 'PING') INSERT INTO `Property` (`Property`,
// `Value`) VALUES ('ProductVersion', '0.1.0')
fn set_property(msi: &mut MsiPackage) {
    let table_name = "Property";
    let columns = vec![
        Column::build("Property").primary_key().string(72),
        Column::build("Value").localizable().string(0),
    ];
    msi.create_table(table_name, columns).unwrap();
    let query = Insert::into(table_name).rows(vec![
        to_values!("DiskPrompt", "PING Installation [1]"),
        to_values!("UpgradeCode", "{*}"),
        to_values!("ALLUSERS", "1"),
        to_values!("Manufacturer", "Manny"),
        to_values!("ProductLanguage", "1033"),
        to_values!("ProductCode", "{328FDEA1-B4AF-4461-91A9-60F0B3C63DB5}"),
        to_values!("ProductName", "PING"),
        to_values!("ProductVersion", "0.1.0"),
    ]);
    msi.insert_rows(query).unwrap()
}

// CREATE TABLE `Registry` (`Registry` CHAR(72) NOT NULL, `Root` INT NOT NULL,
// `Key` CHAR(255) NOT NULL LOCALIZABLE, `Name` CHAR(255) LOCALIZABLE, `Value`
// CHAR(0) LOCALIZABLE, `Component_` CHAR(72) NOT NULL PRIMARY KEY `Registry`)
// INSERT INTO `Registry` (`Registry`, `Root`, `Key`, `Name`, `Value`,
// `Component_`) VALUES ('reg8B8E36E486B293215DD585990A46A8CE', 1,
// 'Software\\manny\\ping.exe', 'installed', '#1', '0')
fn set_registry(msi: &mut MsiPackage) {
    let table_name = "Registry";
    let columns = vec![
        Column::build("Registry").primary_key().string(72),
        Column::build("Root").int16(),
        Column::build("Key").localizable().string(255),
        Column::build("Name").nullable().localizable().string(255),
        Column::build("Value").nullable().localizable().string(0),
        Column::build("Component_").string(72),
    ];
    msi.create_table(table_name, columns).unwrap();
    let query = Insert::into(table_name).rows(vec![to_values!(
        "reg8B8E36E486B293215DD585990A46A8CE",
        1,
        "Software\\manny\\ping.exe",
        "installed",
        "#1",
        "0"
    )]);
    msi.insert_rows(query).unwrap()
}

// CREATE TABLE `ServiceControl` (`ServiceControl` CHAR(72) NOT NULL, `Name`
// CHAR(255) NOT NULL LOCALIZABLE, `Event` INT NOT NULL, `Arguments` CHAR(255)
// LOCALIZABLE, `Wait` INT, `Component_` CHAR(72) NOT NULL PRIMARY KEY
// `ServiceControl`) INSERT INTO `ServiceControl` (`ServiceControl`, `Name`,
// `Event`, `Wait`, `Component_`) VALUES ('9', 'PingService', 163, 1, '6')
fn set_service_control(msi: &mut MsiPackage) {
    let table_name = "ServiceControl";
    let columns = vec![
        Column::build("ServiceControl").primary_key().string(72),
        Column::build("Name").localizable().string(255),
        Column::build("Event").int16(),
        Column::build("Arguments").nullable().localizable().string(255),
        Column::build("Wait").nullable().int16(),
        Column::build("Component_").string(72),
    ];
    msi.create_table(table_name, columns).unwrap();
    let query = Insert::into(table_name).rows(vec![to_values!(
        "9",
        "PingService",
        163,
        Value::Null,
        1,
        "6"
    )]);
    msi.insert_rows(query).unwrap()
}

// CREATE TABLE `ServiceInstall` (`ServiceInstall` CHAR(72) NOT NULL, `Name`
// CHAR(255) NOT NULL, `DisplayName` CHAR(255) LOCALIZABLE, `ServiceType` LONG
// NOT NULL, `StartType` LONG NOT NULL, `ErrorControl` LONG NOT NULL,
// `LoadOrderGroup` CHAR(255), `Dependencies` CHAR(255), `StartName` CHAR(255),
// `Password` CHAR(255), `Arguments` CHAR(255), `Component_` CHAR(72) NOT NULL,
// `Description` CHAR(255) LOCALIZABLE PRIMARY KEY `ServiceInstall`) INSERT INTO
// `ServiceInstall` (`ServiceInstall`, `Name`, `DisplayName`, `ServiceType`,
// `StartType`, `ErrorControl`, `StartName`, `Component_`, `Description`) VALUES
// ('9', 'PingService', 'PingService', 16, 2, 1, 'LocalSystem', '6',
// 'Description for PingService')
fn set_service_install(msi: &mut MsiPackage) {
    let table_name = "ServiceInstall";
    let columns = vec![
        Column::build("ServiceInstall").primary_key().string(72),
        Column::build("Name").string(255),
        Column::build("DisplayName").nullable().localizable().string(255),
        Column::build("ServiceType").int32(),
        Column::build("StartType").int32(),
        Column::build("ErrorControl").int32(),
        Column::build("LoadOrderGroup").nullable().string(255),
        Column::build("Dependencies").nullable().string(255),
        Column::build("StartName").nullable().string(255),
        Column::build("Password").nullable().string(255),
        Column::build("Arguments").nullable().string(255),
        Column::build("Component_").string(72),
        Column::build("Description").nullable().localizable().string(255),
    ];
    msi.create_table(table_name, columns).unwrap();
    let query = Insert::into(table_name).rows(vec![to_values!(
        "9",
        "PingService",
        "PingService",
        16,
        2,
        1,
        Value::Null,
        Value::Null,
        "LocalSystem",
        Value::Null,
        Value::Null,
        "6",
        "Description for PingService"
    )]);
    msi.insert_rows(query).unwrap()
}

// CREATE TABLE `Shortcut` (`Shortcut` CHAR(72) NOT NULL, `Directory_` CHAR(72)
// NOT NULL, `Name` CHAR(128) NOT NULL LOCALIZABLE, `Component_` CHAR(72) NOT
// NULL, `Target` CHAR(72) NOT NULL, `Arguments` CHAR(255), `Description`
// CHAR(255) LOCALIZABLE, `Hotkey` INT, `Icon_` CHAR(72), `IconIndex` INT,
// `ShowCmd` INT, `WkDir` CHAR(72), `DisplayResourceDLL` CHAR(255),
// `DisplayResourceId` INT, `DescriptionResourceDLL` CHAR(255),
// `DescriptionResourceId` INT PRIMARY KEY `Shortcut`) INSERT INTO `Shortcut`
// (`Shortcut`, `Directory_`, `Name`, `Component_`, `Target`, `Description`,
// `Icon_`, `IconIndex`, `WkDir`) VALUES ('ApplicationDesktopShortcut',
// 'DesktopFolder', 'PingShortcut', '0', '[INSTALLDIR]bin/ping.exe', 'Run Ping',
// '1', 0, 'INSTALLDIR')
fn set_shortcut(msi: &mut MsiPackage) {
    let table_name = "Shortcut";
    let columns = vec![
        Column::build("Shortcut").primary_key().string(72),
        Column::build("Directory_").string(72),
        Column::build("Name").localizable().string(128),
        Column::build("Component_").string(72),
        Column::build("Target").string(72),
        Column::build("Arguments").nullable().string(255),
        Column::build("Description").localizable().nullable().string(255),
        Column::build("Hotkey").nullable().int16(),
        Column::build("Icon_").nullable().string(72),
        Column::build("IconIndex").nullable().int16(),
        Column::build("ShowCmd").nullable().int16(),
        Column::build("WkDir").nullable().string(72),
        Column::build("DisplayResourceDLL").nullable().string(255),
        Column::build("DisplayResourceId").nullable().int16(),
        Column::build("DescriptionResourceDLL").nullable().string(255),
        Column::build("DescriptionResourceId").nullable().int16(),
    ];
    msi.create_table(table_name, columns).unwrap();
    let query = Insert::into(table_name).rows(vec![to_values!(
        "ApplicationDesktopShortcut",
        "DesktopFolder",
        "PingShortcut",
        "0",
        "[INSTALLDIR]bin/ping.exe",
        Value::Null,
        "Run Ping",
        Value::Null,
        "1",
        0,
        Value::Null,
        "INSTALLDIR",
        Value::Null,
        Value::Null,
        Value::Null,
        Value::Null
    )]);
    msi.insert_rows(query).unwrap()
}
