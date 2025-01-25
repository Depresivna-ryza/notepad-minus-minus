use std::{cmp::{max, min}, fs::read_to_string, path::{Path, PathBuf}};

use itertools::Itertools;

use super::event::Event;
use ropey::Rope;


#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Caret{
    pub ln: usize,
    pub col: usize,
}

impl Caret{
    pub fn from(l: usize, c: usize) -> Self{
        Self{
            ln: l,
            col: c,
        }
    }
    pub fn new() -> Self{
        Self{
            ln: 0,
            col: 0,
        }
    }

    // pub fn from_rope 
}

#[derive(Debug, Clone, PartialEq)]
pub struct TextFile {
    pub path: PathBuf,
    // pub content: Vec<Vec<char>>,
    // pub caret: Caret,
    pub rope: Rope,
    pub char_idx: usize,
}

impl TextFile {
    pub fn new(path: PathBuf) -> Self {
        let mut content = read_to_string(&path).ok().unwrap_or(String::new()).replace("\r\n", "\n");

        // add newline if the contents last char is not newline
        if !content.ends_with('\n') {
            content.push('\n');
        }

        // let mut content = content
        //     .lines()
        //     .map(|line| line.chars().collect_vec())
        //     .collect_vec();

        // if content.is_empty() {
        //     content = vec![Vec::new()];
        // }

        Self {
            path: path,
            rope: Rope::from_str(&content),
            char_idx: 0,
        }
    }

    pub fn to_string(&self) -> String {
        // self.content
        //     .iter()
        //     .map(|line| line.iter().collect())
        //     .collect::<Vec<String>>()
        //     .join("\n")
        self.rope.to_string()
    }

    // pub fn lines(&self) -> Vec<String> {
    //     // self.content
    //     //     .iter()
    //     //     .map(|line| line.iter().collect())
    //     //     .collect()
    //     self.rope.lines().map(|line| line.to_string()).collect()
    // }

    pub fn chars(&self) -> Vec<Vec<char>> {
        // self.content
        //     .iter()
        //     .map(|line| line.iter().collect())
        //     .collect()
        self.rope.lines().map(|line| line.chars().collect::<Vec<char>>()).filter(|l| !l.is_empty()).collect()
    }

    pub fn caret_move_left(&mut self) {
        // match (self.caret.ln, self.caret.col) {
        //     (_, c) if c > 0 => self.caret.col -= 1,
        //     (l, _) if l > 0 => {
        //         self.caret.ln -= 1;
        //         self.caret.col = match self.get_cols(self.caret.ln){
        //             0 => 0,
        //             c => c - 1
        //         }
        //     }
        //     _ => {}
        // }

        self.char_idx = match self.char_idx {
            0 => 0,
            i => i - 1,
        }
    }

    pub fn caret_move_right(&mut self) {
        // match (self.caret.ln, self.caret.col) {
        //     (l, c) if c < self.get_cols(l) => self.caret.col += 1,
        //     (l, _) if l + 1 < self.content.len() => {
        //         self.caret.ln += 1;
        //         self.caret.col = 0;
        //     }
        //     _ => {}
        // }
        self.char_idx = match self.char_idx {
            i if i + 1 < self.rope.len_chars() => i + 1,
            _ => self.rope.len_chars() - 1,
        }
    }

    pub fn get_caret(&self) -> Caret {
        let mut char_sum = 0;


        for (i, line) in self.rope.lines().enumerate() {
            char_sum += line.len_chars();
            if char_sum > self.char_idx {
                let caret_ln = i;
                let caret_col = self.char_idx - (char_sum - line.len_chars());
                return Caret::from(caret_ln, caret_col);
            }
        }

        Caret::from(self.rope.len_lines() - 1, self.rope.line(self.rope.len_lines() - 1).len_chars())
    }

    pub fn get_char_idx(&self, caret: Caret) -> usize {
        let mut char_sum = 0;
        for (i, line) in self.rope.lines().enumerate() {
            if i == caret.ln {
                return char_sum + caret.col;
            }
            char_sum += line.len_chars();
        }
        panic!("invalid caret in rope");

    }

    pub fn caret_move_down(&mut self) {
        let caret = self.get_caret();

        if caret.ln + 1 == self.rope.len_lines() {
            return;
        }

        let Some(line) = self.rope.get_line(caret.ln) else {
            return;
        };

        let Some(next_line) = self.rope.get_line(caret.ln + 1) else {
            return;
        };

        if next_line.len_chars() == 0 {
            self.char_idx = self.rope.len_chars() - 1;
            return;
        }
        

        // self.char_idx = min(self.char_idx + next_line.len_chars(), self.content.len_chars());

        if caret.col < next_line.len_chars() {
            self.char_idx += line.len_chars();
        } else {
            self.char_idx = self.char_idx - caret.col + line.len_chars() + next_line.len_chars() - 1;
        }
    }

    pub fn caret_move_up(&mut self) {
        // if self.caret.ln > 0 {
        //     self.caret.ln -= 1;
        //     self.caret.col = self.caret.col.min(self.get_cols(self.caret.ln));
        // }

        let caret = self.get_caret();

        if caret.ln == 0 {
            self.char_idx = 0;
            return;
        }

        let Some(line) = self.rope.get_line(caret.ln) else {
            return;
        };

        let Some(prev_line) = self.rope.get_line(caret.ln - 1) else {
            return;
        };

        if caret.col < prev_line.len_chars() {
            self.char_idx -= prev_line.len_chars();
        } else {
            self.char_idx -= caret.col + 1;
        }
    }
   
    pub fn backspace(&mut self) {
        // let Some(line) = self.content.get(self.caret.ln) else { 
        //     return; 
        // };

        // match self.caret.col {
        //     0 => {

        //     }
        //     c => {

        //     }
        // }
        

        // self.apply_event(Event::RemoveChar(self.caret));

        // match (self.caret.ln, self.caret.col) {
        //     (l, c) if c > 0 => {
        //         self.content[l].remove(c - 1);
        //         self.caret_move_left();
        //     }
        //     (l, _) if l > 0 => {
        //         let right_part: Vec<char> = self.content[l].drain(..).collect();

        //         self.caret.ln -= 1;
        //         self.caret.col = self.get_cols(self.caret.ln);
        //         self.content[self.caret.ln].extend(right_part);
        //         self.content.remove(self.caret.ln + 1);
        //     }
        //     (_,_) => {}
        // }


        if self.char_idx == 0 {
            return;
        }

        self.rope.remove(self.char_idx - 1 ..self.char_idx);
        self.caret_move_left();
    }

    pub fn delete(&mut self) {
        if self.char_idx == self.rope.len_chars() - 1 {
            return;
        }

        self.rope.remove(self.char_idx..self.char_idx + 1);
    }

    pub fn insert_char(&mut self, c: char) {
        // self.apply_event(Event::AddChar(c, self.caret));

        // self.content[caret.ln].insert(caret.col, c);
        // self.caret_move_right();

        self.rope.insert_char(self.char_idx, c);
        self.caret_move_right();
    }

    pub fn insert_newline(&mut self) {
        // let right_part: Vec<char> = self.rope[self.caret.ln]
        //     .drain(self.caret.col..)
        //     .collect();
        // self.rope.insert(self.caret.ln + 1, right_part);

        // self.caret_move_right();

        self.insert_char('\n');
    }

    pub fn set_caret_position(&mut self, line: usize, column: usize) {
        // self.caret.ln = max(0, min(line, self.rope.len() - 1));
        // self.caret.col = max(0, min(column, self.get_cols(self.caret.ln)));

        self.char_idx = self.get_char_idx(Caret::from(line, column));

    }


    pub fn apply_event(&mut self, event: Event) {
        // match event {
        //     Event::AddChar(c, caret) => {
        //         self.rope[caret.ln].insert(caret.col, c);
        //         self.set_caret_position(caret.ln, caret.col);
        //         self.caret_move_right();

        //     }
        //     Event::RemoveChar(_, _) => {

        //     }
        //     Event::AddNewLine(_) => {

        //     }
        // }
    }
}
