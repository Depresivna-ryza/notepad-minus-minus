use std::path::PathBuf;

use itertools::Itertools;

#[derive(PartialEq, Clone, Debug)]
pub struct FileSystem {
    pub root: Dir,
}

impl FileSystem {
    pub fn new(root: Dir) -> Self {
        Self { root }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Dir {
    pub path: PathBuf,
    pub children: DirectoryItems,
}

impl Dir {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            children: DirectoryItems::ClosedDirectory,
        }
    }

    pub fn open(&mut self) {
        let Ok(read_dir) = self.path.read_dir() else {
            return;
        };

        let items = read_dir
            .filter_map(|entry| entry.ok())
            .filter_map(|entry| match entry.file_type() {
                Ok(file_type) if file_type.is_dir() => {
                    let dir = Dir::new(entry.path());
                    Some(DirectoryItem::Directory(dir))
                }
                Ok(file_type) if file_type.is_file() => Some(DirectoryItem::File(entry.path())),
                _ => None,
            })
            .collect_vec();

        self.children = DirectoryItems::OpenedDirectory(items);
    }
    pub fn close(&mut self) {
        self.children = DirectoryItems::ClosedDirectory;
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum DirectoryItems {
    ClosedDirectory,
    OpenedDirectory(Vec<DirectoryItem>),
}

#[derive(PartialEq, Clone, Debug)]
pub enum DirectoryItem {
    File(PathBuf),
    Directory(Dir),
}
