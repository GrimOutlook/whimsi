use std::{any, path::PathBuf};

use anyhow::ensure;
use itertools::Itertools;

use crate::{
    tables::{directory::DirectoryError, file::helper::File},
    types::helpers::{directory_item::DirectoryItem, filename::Filename},
};

use super::helper::Directory;

// TODO: If the `getset` crate ever supports Traits, use them here. I should not have to manually
// make getters just because they are contained in traits.
#[ambassador::delegatable_trait]
pub trait DirectoryKind: Clone + std::fmt::Display {
    fn name_conflict(&self, other: &Self) -> bool;
    fn contents(&self) -> &Vec<DirectoryItem>;
    fn contents_mut(&mut self) -> &mut Vec<DirectoryItem>;

    fn with_contents(mut self, contents: &mut Vec<DirectoryItem>) -> Self {
        self.add_contents(contents);
        self
    }

    fn add_contents(&mut self, contents: &mut Vec<DirectoryItem>) {
        self.contents_mut().append(contents);
    }

    fn with_item(mut self, item: impl Into<DirectoryItem>) -> anyhow::Result<Self> {
        self.add_item(item);
        Ok(self)
    }

    fn add_item(&mut self, item: impl Into<DirectoryItem>) -> anyhow::Result<()> {
        let item = item.into();
        match item {
            DirectoryItem::File(ref file) => {
                ensure!(
                    self.contained_files()
                        .iter()
                        .find(|other| other.name() == file.name())
                        .is_none(),
                    DirectoryError::DuplicateFile {
                        name: file.name().to_string()
                    }
                )
            }
            DirectoryItem::Directory(ref directory) => {
                ensure!(
                    self.contained_directories()
                        .iter()
                        .find(|other| directory.name_conflict(other))
                        .is_none(),
                    DirectoryError::DuplicateDirectory {
                        name: directory.to_string()
                    }
                )
            }
        }
        self.contents_mut().push(item.into());
        Ok(())
    }

    fn with_path_contents(mut self, path: PathBuf) -> anyhow::Result<Self> {
        self.add_path_contents(path)?;
        Ok(self)
    }

    fn add_path_contents(&mut self, path: PathBuf) -> anyhow::Result<()> {
        let dir = Directory::try_from(path)?;
        self.add_contents(&mut dir.contents().clone());

        Ok(())
    }

    fn contained_directories(&self) -> Vec<&Directory> {
        self.contents()
            .iter()
            .filter_map(|node| node.try_as_directory_ref())
            .collect_vec()
    }

    fn contained_files(&self) -> Vec<File> {
        self.contents()
            .iter()
            .filter_map(|node| node.try_as_file_ref())
            .cloned()
            .collect_vec()
    }

    fn contained_directory_by_name(&self, name: &str) -> Option<&Directory> {
        self.contained_directories()
            .into_iter()
            .find(|dir| dir.name().long().to_string() == name)
    }

    fn insert_dir_strict(&mut self, name: &str) -> anyhow::Result<Directory> {
        let new_filename = Filename::parse(name)?;
        self.insert_dir_filename(new_filename)
    }

    fn insert_dir(&mut self, name: &str) -> anyhow::Result<Directory> {
        let new_filename = Filename::parse(name)?;
        self.insert_dir_filename(new_filename)
    }

    fn insert_dir_filename(&mut self, filename: Filename) -> anyhow::Result<Directory> {
        let contents = self.contents();
        let contained_dirs = contents
            .iter()
            .filter_map(|node| node.try_as_directory_ref());
        ensure!(
            // TODO: This clone might be able to be removed with some reordering.
            !contained_dirs.clone().any(|dir| *dir.name() == filename),
            DirectoryError::DuplicateDirectory {
                name: filename.to_string()
            }
        );

        let new_dir = Directory::from(filename);
        self.contents_mut().push(new_dir.clone().into());
        Ok(new_dir)
    }

    fn print_structure(&self) {
        self.print_content_structure(0)
    }

    fn print_content_structure(&self, depth: usize) {
        let delimiter = "|- ";
        let depth_str = |x| " ".repeat(x * delimiter.len());
        if depth == 0 {
            println!("{self}/");
        } else {
            println!("{}{delimiter}{self}/", depth_str(depth))
        }
        let files = self.contained_files().into_iter().sorted();
        let directories = self.contained_directories().into_iter().sorted();
        for file in files {
            println!("{}{delimiter}{file}", depth_str(depth + 1));
        }
        for directory in directories {
            directory.print_content_structure(depth + 1);
        }
    }
}

#[macro_export]
macro_rules! implement_directory_kind_boilerplate {
    () => {
        fn contents(&self) -> &Vec<DirectoryItem> {
            &self.contained
        }

        fn contents_mut(&mut self) -> &mut Vec<DirectoryItem> {
            &mut self.contained
        }
    };
}
