use std::{cmp::{max, min}, path::PathBuf};

#[derive(PartialEq, Clone, Debug)]
pub struct Tabs {
    pub opened_tabs: Vec<PathBuf>,
    pub current_file: Option<PathBuf>,
}

impl Tabs {
    pub fn new() -> Self {
        Self {
            opened_tabs: Vec::new(),
            current_file: None,
        }
    }

    pub fn open_tab(&mut self, path: PathBuf) {
        self.set_current_file(path.clone());

        if self.opened_tabs.contains(&path) {
            return;
        }

        self.opened_tabs.push(path.clone());
    }

    pub fn close_tab(&mut self, path: PathBuf) {
        for (i, tab) in self.opened_tabs.iter().enumerate() {
            if tab == &path {
                self.opened_tabs.remove(i);
                self.current_file = match self.opened_tabs.len() {
                    0 => None,
                    l => self.opened_tabs.get(min(i, l - 1)).cloned(),
                };

                break;
            }
        }
    }

    pub fn set_current_file(&mut self, path: PathBuf) {
        self.current_file = Some(path.clone());
    }
}

