use std::fs::File;

use whimsi_lib::{
    MsiBuilder,
    tables::directory::{helper::Directory, kind::DirectoryKind},
    types::properties::system_folder::SystemFolder,
};
fn main() {
    let mut builder = MsiBuilder::default();
    let mut prog = Directory::from(SystemFolder::ProgramFilesFolder);
    let manny = Directory::new("manny").unwrap();
    manny.add_path_contents("./reference/root_dir/").unwrap();
    prog.add_item(manny);
    let file = File::create("test.msi").unwrap();
    builder.build(file).unwrap();
    drop(file);
}
