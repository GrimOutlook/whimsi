use std::fs::File;

use tracing::level_filters::LevelFilter;
use whimsi_lib::builder::MsiBuilder;
use whimsi_lib::types::properties::system_folder::SystemFolder;
fn main() {
    tracing_subscriber::fmt().with_max_level(LevelFilter::TRACE).init();
    let file = File::options()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open("test.msi")
        .expect("Failed to open file");

    let mut builder = MsiBuilder::default();
    let manny_id = builder
        .add_directory("manny", SystemFolder::ProgramFilesFolder)
        .expect("Failed to create directory");

    builder
        .with_path_contents("./examples/reference/root_dir/", manny_id)
        .expect("Failed to add path to MSIBuilder")
        .build(file)
        .expect("Failed to build MSI");
}
