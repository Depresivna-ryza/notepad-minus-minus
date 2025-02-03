use std::{cmp::min, path::PathBuf};

use super::text::TextFile;

#[derive(PartialEq, Clone, Debug)]
pub struct Tab {
    pub file: TextFile,
    pub exists: bool,
}

impl Tab {
    pub fn new(file: TextFile) -> Self {
        Self { file, exists: true }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Tabs {
    pub opened_tabs: Vec<Tab>,
    pub current_file: Option<PathBuf>,
}

impl Default for Tabs {
    fn default() -> Self {
        Self::new()
    }
}

impl Tabs {
    pub fn new() -> Self {
        Self {
            opened_tabs: Vec::new(),
            current_file: None,
        }
    }

    pub fn get_current_file(&self) -> Option<TextFile> {
        self.current_file
            .as_ref()
            .and_then(|path| self.get_file(path))
    }

    pub fn get_current_file_mut(&mut self) -> Option<&mut TextFile> {
        self.current_file.as_ref().and_then(|path| {
            self.opened_tabs
                .iter_mut()
                .find(|file| &file.file.path == path)
                .map(|file| &mut file.file)
        })
    }

    pub fn get_file(&self, path: &PathBuf) -> Option<TextFile> {
        self.opened_tabs
            .iter()
            .find(|file| &file.file.path == path)
            .map(|file| file.file.clone())
    }

    pub fn open_tab(&mut self, path: PathBuf) {
        self.set_current_file(path.clone());

        if self.get_file(&path).is_none() {
            self.opened_tabs.push(Tab::new(TextFile::new(path)));
        }
    }

    pub fn close_tab(&mut self, path: PathBuf) {
        for (i, tab) in self.opened_tabs.iter().enumerate() {
            if tab.file.path == path {
                self.opened_tabs.remove(i);
                self.current_file = match self.opened_tabs.len() {
                    0 => None,
                    l => self
                        .opened_tabs
                        .get(min(i, l - 1))
                        .map(|f| f.file.path.clone()),
                };

                break;
            }
        }
    }

    pub fn update_existance(&mut self, path: PathBuf, exists: bool) {
        if let Some(x) = self
            .opened_tabs
            .iter_mut()
            .find(|file| file.file.path == path)
        {
            x.exists = exists;
        }
    }

    pub fn set_current_file(&mut self, path: PathBuf) {
        self.current_file = Some(path.clone());
    }
}
