use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub enum FileSystemItem {
    File(PathBuf),
    Directory(Directory),
}

impl PartialEq for FileSystemItem {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (FileSystemItem::File(path1), FileSystemItem::File(path2)) => path1 == path2,
            (FileSystemItem::Directory(dir1), FileSystemItem::Directory(dir2)) => {
                dir1.path == dir2.path
            }
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FileSystem {
    root: Option<Directory>,
    focused_fs_item: Option<PathBuf>,
}

impl Default for FileSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl FileSystem {
    pub fn new() -> Self {
        FileSystem {
            root: None,
            focused_fs_item: None,
        }
    }

    pub fn from(root_path: &Path) -> Self {
        let root_directory = Directory::from(root_path);
        FileSystem {
            root: Some(root_directory),
            focused_fs_item: None,
        }
    }

    pub fn get_root(&self) -> Option<&Directory> {
        self.root.as_ref()
    }

    fn find_directory_mut<'a>(&'a mut self, path: &PathBuf) -> Option<&'a mut Directory> {
        if let Some(root) = &mut self.root {
            return root.find_directory_recursive_mut(path);
        }
        None
    }

    fn find_directory<'a>(&'a self, path: &PathBuf) -> Option<&'a Directory> {
        if let Some(root) = &self.root {
            return root.find_directory_recursive(path);
        }
        None
    }

    pub fn change_focus(&mut self, path: &Path) {
        self.focused_fs_item = Some(path.to_path_buf());
    }

    pub fn clear_focus(&mut self) {
        self.focused_fs_item = None;
    }

    pub fn reload(&mut self) {
        if let Some(root) = &mut self.root {
            root.reload_children();
        }
    }

    pub fn is_focused(&self, path: &PathBuf) -> bool {
        self.focused_fs_item.as_ref() == Some(path)
    }

    pub fn open_close_directory(&mut self, path: &PathBuf) {
        if let Some(directory) = self.find_directory_mut(path) {
            directory.open_close();
        }
    }

    pub fn reload_directory(&mut self, path: &PathBuf) {
        if let Some(directory) = self.find_directory_mut(path) {
            directory.reload_direct_children();
        }
    }

    pub fn get_directory_name(&self, path: &PathBuf) -> String {
        if let Some(directory) = self.find_directory(path) {
            return directory.get_name();
        }
        String::new()
    }

    pub fn directory_is_opened(&self, path: &PathBuf) -> bool {
        if let Some(directory) = self.find_directory(path) {
            return directory.is_opened();
        }
        false
    }

    pub fn get_directory_children(&self, path: &PathBuf) -> Vec<FileSystemItem> {
        if let Some(directory) = self.find_directory(path) {
            return directory.get_children();
        }
        Vec::new()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Directory {
    path: PathBuf,
    children: Vec<FileSystemItem>,
    opened: bool,
}

impl Directory {
    pub fn from(path: &Path) -> Self {
        Directory {
            path: path.to_path_buf(),
            children: Vec::new(),
            opened: false,
        }
    }

    pub fn reload_children(&mut self) {
        self.reload_direct_children();

        for child in self.children.iter_mut() {
            if let FileSystemItem::Directory(dir) = child {
                if dir.is_opened() {
                    dir.reload_children();
                }
            }
        }
    }

    pub fn reload_direct_children(&mut self) {
        let mut updated_children = Vec::new();

        if let Ok(entries) = fs::read_dir(&self.path) {
            for entry in entries.flatten() {
                let entry_path = entry.path();
                if entry_path.is_dir() {
                    updated_children.push(FileSystemItem::Directory(Directory::from(&entry_path)));
                } else {
                    updated_children.push(FileSystemItem::File(entry_path));
                }
            }
        }

        self.sync_children(updated_children);
    }

    fn sync_children(&mut self, updated_children: Vec<FileSystemItem>) {
        let mut new_children = Vec::new();

        for child in self.children.iter() {
            if updated_children.contains(child) {
                new_children.push(child.clone());
            }
        }

        for child in updated_children {
            if !new_children.contains(&child) {
                new_children.push(child);
            }
        }

        self.children = new_children;
    }

    fn find_directory_recursive_mut<'a>(&'a mut self, path: &PathBuf) -> Option<&'a mut Directory> {
        if *path == self.path {
            return Some(self);
        }

        for child in self.children.iter_mut() {
            if let FileSystemItem::Directory(child_dir) = child {
                if let Some(fs_item) = child_dir.find_directory_recursive_mut(path) {
                    return Some(fs_item);
                }
            }
        }
        None
    }

    fn find_directory_recursive<'a>(&'a self, path: &PathBuf) -> Option<&'a Directory> {
        if *path == self.path {
            return Some(self);
        }

        for child in self.children.iter() {
            if let FileSystemItem::Directory(child_dir) = child {
                if let Some(fs_item) = child_dir.find_directory_recursive(path) {
                    return Some(fs_item);
                }
            }
        }
        None
    }

    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }

    pub fn get_name(&self) -> String {
        self.path.file_name().unwrap().to_str().unwrap().to_string()
    }

    pub fn open_close(&mut self) {
        if self.is_opened() {
            self.opened = false;
        } else {
            self.opened = true;
            self.reload_direct_children();
        }
    }

    pub fn is_opened(&self) -> bool {
        self.opened
    }

    pub fn get_children(&self) -> Vec<FileSystemItem> {
        self.children.clone()
    }
}
