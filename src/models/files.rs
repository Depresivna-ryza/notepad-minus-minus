use std::path::PathBuf;
use itertools::Itertools;

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

#[derive(PartialEq, Clone, Debug)]
pub struct FileSystem {
    root: Option<Dir>,
    focus: Option<PathBuf>,
}

impl FileSystem {
    pub fn from(root: Dir) -> Self {
        Self { root: Some(root), focus: None, }
    }

    pub fn new() -> Self {
        Self { root: None, focus: None, }
    }

    pub fn find(&mut self, path: &PathBuf) {
        if let Some(ref mut root) = self.root {
            root.find(path);
        }
    }
    
    pub fn change_focus(&mut self, path: &PathBuf) {
        self.focus = Some(path.clone());
    }
    
    pub fn clear_focus(&mut self) {
        self.focus = None;
    }
    
    pub fn is_focused(&self, path: &PathBuf) -> bool {
        self.focus.as_ref() == Some(path)
    }

    pub fn get_root(&self) -> Option<&Dir> {
        self.root.as_ref()
    }

    pub fn get_root_path(&self) -> Option<&PathBuf> {
        self.root.as_ref().map(|root| &root.path)
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

    pub fn find(&mut self, path: &PathBuf) {
        if path == &self.path {
            self.open_close();
            return;
        }

        if let DirectoryItems::OpenedDirectory(ref mut items) = self.children {
            for item in items.iter_mut() {
                match item {
                    DirectoryItem::Directory(dir) => {
                        dir.find(path);
                    }
                    DirectoryItem::File(file) => {
                        if file == path {
                            return;
                        }
                    }
                }
            }
        }
    }

    pub fn open_close(&mut self) {
        match self.children {
            DirectoryItems::ClosedDirectory => self.open(),
            DirectoryItems::OpenedDirectory(_) => self.close(),
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
