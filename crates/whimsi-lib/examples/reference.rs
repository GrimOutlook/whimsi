use std::fs::File;

use whimsi_msi::Language;
use tracing::level_filters::LevelFilter;
use whimsi_lib::builder::MsiBuilder;
use whimsi_lib::tables::meta::MetaInformation;
use whimsi_lib::types::helpers::architecture::MsiArchitecture;
use whimsi_lib::types::properties::system_folder::SystemFolder;
fn main() {
    tracing_subscriber::fmt().with_max_level(LevelFilter::TRACE).init();
    let file = File::options()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open("/tmp/test.msi")
        .expect("Failed to open file");

    let meta =
        MetaInformation::new(whimsi_msi::PackageType::Installer, "PING".to_string())
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
        .build(file)
        .expect("Failed to build MSI");
}
