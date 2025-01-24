use std::{fs::read_to_string, path::{Path, PathBuf}};

use itertools::Itertools;

#[derive(Debug, Clone, PartialEq)]
pub struct TextFile {
    pub path: PathBuf,
    pub content: Vec<Vec<char>>,
    pub caret_line: usize,
    pub caret_column: usize,
}

impl TextFile {
    pub fn new(path: PathBuf) -> Self {
        let content = read_to_string(&path).ok().unwrap_or(String::new());

        let mut content = content
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec();

        if content.is_empty() {
            content = vec![Vec::new()];
        }

        Self {
            path: path,
            content: content.clone(),
            caret_line: 0,
            caret_column: 0,
        }
    }

    pub fn to_string(&self) -> String {
        self.content
            .iter()
            .map(|line| line.iter().collect())
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn get_cols(&self, line: usize) -> usize {
        self.content.get(line).map(|line| line.len()).unwrap()
    }

    pub fn lines(&self) -> Vec<String> {
        self.content
            .iter()
            .map(|line| line.iter().collect())
            .collect()
    }

    pub fn lines_chars(&self) -> Vec<Vec<&char>> {
        self.content
            .iter()
            .map(|line| line.iter().collect())
            .collect()
    }

    pub fn caret_move_left(&mut self) {
        match (self.caret_line, self.caret_column) {
            (_, c) if c > 0 => self.caret_column -= 1,
            (l, _) if l > 0 => {
                self.caret_line -= 1;
                self.caret_column = match self.get_cols(self.caret_line){
                    0 => 0,
                    c => c - 1
                }
            }
            _ => {}
        }
    }

    pub fn caret_move_right(&mut self) {
        match (self.caret_line, self.caret_column) {
            (l, c) if c < self.get_cols(l) => self.caret_column += 1,
            (l, _) if l + 1 < self.content.len() => {
                self.caret_line += 1;
                self.caret_column = 0;
            }
            _ => {}
        }
    }

    pub fn caret_move_down(&mut self) {
        if self.caret_line + 1 < self.content.len() {
            self.caret_line += 1;
            self.caret_column = self.caret_column.min(self.get_cols(self.caret_line));
        }
    }

    pub fn caret_move_up(&mut self) {
        if self.caret_line > 0 {
            self.caret_line -= 1;
            self.caret_column = self.caret_column.min(self.get_cols(self.caret_line));
        }
    }

    pub fn remove_char(&mut self) {
        match (self.caret_line, self.caret_column) {
            (l, c) if c > 0 => {
                self.content[l].remove(c - 1);
                self.caret_move_left();
            }
            (l, _) if l > 0 => {
                let right_part: Vec<char> = self.content[l].drain(..).collect();

                self.caret_line -= 1;
                self.caret_column = self.get_cols(self.caret_line);
                self.content[self.caret_line].extend(right_part);
                self.content.remove(self.caret_line + 1);
            }
            (_,_) => {}
        }
    }

    pub fn insert_char(&mut self, c: char) {
        self.content[self.caret_line].insert(self.caret_column, c);
        self.caret_move_right();
    }

    pub fn insert_newline(&mut self) {
        let right_part: Vec<char> = self.content[self.caret_line]
            .drain(self.caret_column..)
            .collect();
        self.content.insert(self.caret_line + 1, right_part);

        self.caret_move_right();
    }
}
