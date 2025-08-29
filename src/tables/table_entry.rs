use super::directory::dao::DirectoryDao;

#[derive(Clone, Debug)]
pub enum TableEntry {
    Directory(DirectoryDao),
}
