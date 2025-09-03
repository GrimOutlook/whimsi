use std::fs::File;

use whimsi_lib::{
    MsiBuilder,
    tables::directory::{
        helper::Directory, kind::DirectoryKind, system_directory::SystemDirectory,
    },
    types::properties::system_folder::SystemFolder,
};
fn main() {
    let mut prog = SystemDirectory::from(SystemFolder::ProgramFilesFolder);
    let mut manny = Directory::new("manny").unwrap();
    manny
        .add_path_contents("./reference/root_dir/".into())
        .unwrap();
    prog.add_item(manny).unwrap();
    let builder = MsiBuilder::default().with_directory(prog).unwrap();
    let file = File::create("test.msi").unwrap();
    builder.build(file).unwrap();
}
