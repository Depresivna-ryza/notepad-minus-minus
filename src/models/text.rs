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
}

#[derive(Debug, Clone, PartialEq)]
pub struct TextFile {
    pub path: PathBuf,
    pub rope: Rope,
    pub char_idx: usize,

    event_history: Vec<Event>,
    history_idx: usize,
    pub dirty_changes: Option<usize>,
}

impl TextFile {
    pub fn new(path: PathBuf) -> Self {
        let mut content = read_to_string(&path).ok().unwrap_or(String::new()).replace("\r\n", "\n");

        if !content.ends_with('\n') {
            content.push('\n');
        }

        Self {
            path: path,
            rope: Rope::from_str(&content),
            char_idx: 0,
            event_history: Vec::new(),
            history_idx: 0,
            dirty_changes: None,
        }
    }

    pub fn save_to_file(&mut self) {
        let content = self.to_string();

        match std::fs::write(&self.path, content) {
            Ok(_) => {
                self.dirty_changes = None;
            }
            Err(_e) => {
            }
        }
        
    }

    pub fn to_string(&self) -> String {
        self.rope.to_string()
    }

    pub fn chars(&self) -> Vec<Vec<char>> {
        self.rope.lines().map(|line| line.chars().collect::<Vec<char>>()).filter(|l| !l.is_empty()).collect()
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

    pub fn set_caret_position(&mut self, line: usize, column: usize) {
        self.char_idx = self.get_char_idx(Caret::from(line, column));
    }
    
    pub fn caret_move_left(&mut self) {
        self.char_idx = match self.char_idx {
            0 => 0,
            i => i - 1,
        }
    }

    pub fn caret_move_right(&mut self) {

        self.char_idx = match self.char_idx {
            i if i + 1 < self.rope.len_chars() => i + 1,
            _ => self.rope.len_chars() - 1,
        }
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

        if caret.col < next_line.len_chars() {
            self.char_idx += line.len_chars();
        } else {
            self.char_idx = self.char_idx - caret.col + line.len_chars() + next_line.len_chars() - 1;
        }
    }

    pub fn caret_move_up(&mut self) {
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
        if self.char_idx == 0 {
            return;
        }

        // self.rope.remove(self.char_idx - 1 .. self.char_idx);
        self.apply_new_event(Event::RemoveChar(self.rope.char(self.char_idx - 1), self.char_idx - 1));

        // self.caret_move_left();
    }

    pub fn delete(&mut self) {
        if self.char_idx == self.rope.len_chars() - 1 {
            return;
        }

        // self.rope.remove(self.char_idx..self.char_idx + 1);
        self.apply_new_event(Event::RemoveChar(self.rope.char(self.char_idx), self.char_idx));
    }

    pub fn insert_char(&mut self, c: char) {
        // self.rope.insert_char(self.char_idx, c);
        self.apply_new_event(Event::AddChar(c, self.char_idx));
        // self.caret_move_right();
    }

    pub fn insert_newline(&mut self) {
        self.insert_char('\n');
    }

    pub fn apply_new_event(&mut self, event: Event) {
        self.event_history.truncate(self.history_idx);
        self.event_history.push(event.clone());
        self.history_idx += 1;
        self.apply_event(event);

        self.dirty_changes = Some(self.dirty_changes.map(|d| d + 1).unwrap_or(1));
    }

    pub fn apply_event(&mut self, event: Event) {
        match event {
            Event::AddChar(c, idx) => {
                self.rope.insert_char(idx, c);
                self.char_idx = idx;
                self.caret_move_right();
            }
            Event::RemoveChar(_c, idx) => {
                self.rope.remove(idx .. idx + 1);
                self.char_idx = idx;
                // self.caret_move_left();
            }
        }
    }

    pub fn undo_event(&mut self) {
        if self.history_idx == 0 {
            return;
        }

        let Some(event) = self.event_history.get(self.history_idx - 1).cloned() else {
            return;
        };

        match event {
            Event::AddChar(_c, idx) => {
                self.rope.remove(idx .. idx + 1);
                self.char_idx = idx;
                // self.caret_move_left();
            }
            Event::RemoveChar(c, idx) => {
                self.rope.insert_char(idx, c);
                self.char_idx = idx;
                self.caret_move_right();
            }
        }

        self.history_idx -= 1;
        self.dirty_changes = Some(self.dirty_changes.and_then(|d| d.checked_sub(1)).unwrap_or(0));
    }

    pub fn redo_event(&mut self) {
        if self.history_idx == self.event_history.len() {
            return;
        }

        let Some(event) = self.event_history.get(self.history_idx).cloned() else {
            return;
        };

        self.apply_event(event);
        self.history_idx += 1;

        self.dirty_changes = Some(self.dirty_changes.map(|d| d + 1).unwrap_or(1));
    }
}
