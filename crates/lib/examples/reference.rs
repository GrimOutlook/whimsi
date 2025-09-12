use std::fs::File;

use tracing::level_filters::LevelFilter;
use whimsi_lib::builder::MsiBuilder;
use whimsi_lib::tables::meta::MetaInformation;
use whimsi_lib::types::helpers::architecture::MsiArchitecture;
use whimsi_lib::types::properties::system_folder::SystemFolder;
use whimsi_msi::Language;
fn main() {
    tracing_subscriber::fmt().with_max_level(LevelFilter::TRACE).init();
    let file = File::options()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open("/tmp/test.msi")
        .expect("Failed to open file");

    let meta = MetaInformation::new(
        whimsi_msi::PackageType::Installer,
        "PING".to_string(),
    )
    .with_author(Some("manny".to_string()))
    .with_languages(vec![Language::from_code(1033)])
    .with_architecture(Some(MsiArchitecture::Intel));

    let mut builder = MsiBuilder::default();
    let manny_id = builder
        .add_directory("manny", SystemFolder::ProgramFilesFolder)
        .expect("Failed to create manny directory");
    let ping_id = builder
        .add_directory("PING", manny_id)
        .expect("Failed to create PING directory");

    builder
        .with_meta(meta)
        .with_path_contents(
            "./crates/whimsi-lib/examples/reference/root_dir/",
            ping_id,
        )
        .expect("Failed to add path to MSIBuilder")
        .with_property("Manufacturer", "MANNY")
        .expect("Failed to set Manufacturer")
        .with_property("ProgramName", "PING")
        .expect("Failed to set ProgramName")
        .with_property("ProductCode", uuid::Uuid::new_v4().braced())
        .expect("Failed to set ProductCode")
        .with_property("ProductLanguage", "1033")
        .expect("Failed to set ProgramLanguage")
        .with_property("ProductVersion", "0.1.0")
        .expect("Failed to set ProductVersion")
        .with_property("UpgradeCode", "{*}")
        .expect("Failed to set UpgradeCode")
        .build(file)
        .expect("Failed to build MSI");
}
