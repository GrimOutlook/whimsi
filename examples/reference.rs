use std::fs::File;

use tracing::level_filters::LevelFilter;
use whimsi_lib::{
    builder::MsiBuilder,
    tables::directory::{
        helper::Directory, kind::DirectoryKind, system_directory::SystemDirectory,
    },
    types::properties::system_folder::SystemFolder,
};
fn main() {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::TRACE)
        .init();
    let manny = Directory::new("manny")
        .expect("Failed to create directory")
        .with_path_contents("./examples/reference/root_dir/".into())
        .expect("Failed to add path contents to directory");
    let prog = SystemDirectory::from(SystemFolder::ProgramFilesFolder)
        .with_item(manny)
        .expect("Failed to add directory to system directory");
    let file = File::options()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open("test.msi")
        .expect("Failed to open file");

    MsiBuilder::default()
        .with_directory(prog)
        .expect("Failed to add directory to MSIBuilder")
        .finish()
        .expect("Failed to finalize MSIBuilder")
        .build(file)
        .expect("Failed to build MSI");
}
