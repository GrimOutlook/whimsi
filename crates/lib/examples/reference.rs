use std::fs::File;
use std::path::PathBuf;

use clap::Parser;
use tracing::level_filters::LevelFilter;
use whimsi_lib::builder::MsiBuilder;
use whimsi_lib::tables::meta::MetaInformation;
use whimsi_lib::types::helpers::architecture::MsiArchitecture;
use whimsi_lib::types::properties::system_folder::SystemFolder;
use whimsi_msi::Language;

#[derive(Parser)]
struct Args {
    output_location: PathBuf,
}
fn main() {
    tracing_subscriber::fmt().with_max_level(LevelFilter::TRACE).init();
    let args = Args::parse();
    let file = File::options()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(args.output_location)
        .expect("Failed to open file");

    let meta = MetaInformation::new(
        whimsi_msi::PackageType::Installer,
        "PING".to_string(),
    )
    .with_author(Some("Manny".to_string()))
    .with_languages(vec![Language::from_code(1033)])
    .with_comments(Some("Summary of PING application".to_string()))
    .with_keywords(vec!["Installer".to_string(), "0.1.0".to_string()])
    .with_architecture(Some(MsiArchitecture::X64));

    let mut builder = MsiBuilder::default();
    let manny_id = builder
        .add_directory("manny", SystemFolder::ProgramFiles64Folder)
        .expect("Failed to create manny directory");
    let ping_id = builder
        .add_directory("PING", manny_id)
        .expect("Failed to create PING directory");

    builder
        .with_meta(meta)
        .with_path_contents(
            "./crates/lib/examples/reference/root_dir/",
            ping_id,
        )
        .expect("Failed to add path to MSIBuilder")
        .with_property("Manufacturer", "Manny")
        .expect("Failed to set Manufacturer")
        .with_property("ProductName", "PING")
        .expect("Failed to set ProductName")
        .with_property(
            "ProductCode",
            uuid::Uuid::new_v4().braced().to_string().to_uppercase(),
        )
        .expect("Failed to set ProductCode")
        .with_property("ProductLanguage", "1033")
        .expect("Failed to set ProgramLanguage")
        .with_property("ProductVersion", "0.1.0")
        .expect("Failed to set ProductVersion")
        .with_property("UpgradeCode", "{*}")
        .expect("Failed to set UpgradeCode")
        .with_property("ALLUSERS", "1")
        .expect("Failed to set ALLUSERS")
        .with_property("DiskPrompt", "PING Installation [1]")
        .expect("Failed to set DISKPROMPT")
        .build(file)
        .expect("Failed to build MSI");
}
