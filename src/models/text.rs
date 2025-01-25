use std::{cmp::{max, min}, fs::read_to_string, path::{Path, PathBuf}};

use itertools::Itertools;


#[derive(Debug, Clone, PartialEq)]
pub struct Caret{
    pub l: usize,
    pub c: usize,
}

impl Caret{
    pub fn from(l: usize, c: usize) -> Self{
        Self{
            l,
            c,
        }
    }
    pub fn new() -> Self{
        Self{
            l: 0,
            c: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TextFile {
    pub path: PathBuf,
    pub content: Vec<Vec<char>>,
    pub caret: Caret,
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
            caret: Caret::new(),
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
        match (self.caret.l, self.caret.c) {
            (_, c) if c > 0 => self.caret.c -= 1,
            (l, _) if l > 0 => {
                self.caret.l -= 1;
                self.caret.c = match self.get_cols(self.caret.l){
                    0 => 0,
                    c => c - 1
                }
            }
            _ => {}
        }
    }

    pub fn caret_move_right(&mut self) {
        match (self.caret.l, self.caret.c) {
            (l, c) if c < self.get_cols(l) => self.caret.c += 1,
            (l, _) if l + 1 < self.content.len() => {
                self.caret.l += 1;
                self.caret.c = 0;
            }
            _ => {}
        }
    }

    pub fn caret_move_down(&mut self) {
        if self.caret.l + 1 < self.content.len() {
            self.caret.l += 1;
            self.caret.c = self.caret.c.min(self.get_cols(self.caret.l));
        }
    }

    pub fn caret_move_up(&mut self) {
        if self.caret.l > 0 {
            self.caret.l -= 1;
            self.caret.c = self.caret.c.min(self.get_cols(self.caret.l));
        }
    }

    pub fn remove_char(&mut self) {
        match (self.caret.l, self.caret.c) {
            (l, c) if c > 0 => {
                self.content[l].remove(c - 1);
                self.caret_move_left();
            }
            (l, _) if l > 0 => {
                let right_part: Vec<char> = self.content[l].drain(..).collect();

                self.caret.l -= 1;
                self.caret.c = self.get_cols(self.caret.l);
                self.content[self.caret.l].extend(right_part);
                self.content.remove(self.caret.l + 1);
            }
            (_,_) => {}
        }
    }

    pub fn insert_char(&mut self, c: char) {
        self.content[self.caret.l].insert(self.caret.c, c);
        self.caret_move_right();
    }

    pub fn insert_newline(&mut self) {
        let right_part: Vec<char> = self.content[self.caret.l]
            .drain(self.caret.c..)
            .collect();
        self.content.insert(self.caret.l + 1, right_part);

        self.caret_move_right();
    }

    pub fn set_caret_position(&mut self, line: usize, column: usize) {
        self.caret.l = max(0, min(line, self.content.len() - 1));
        self.caret.c = max(0, min(column, self.get_cols(self.caret.l)));
    }
}
