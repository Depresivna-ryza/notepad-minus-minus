use std::{cmp::{max, min}, fs::read_to_string, path::PathBuf};

use itertools::Itertools;

use super::event::Event;
use ropey::{iter::Lines, Rope};


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

    pub selection: Option<(usize, usize)>,
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
            selection: None,
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

    pub fn chars(&self) -> Lines {
        self.rope.lines()
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

    pub fn get_caret_from_idx(&self, idx: usize) -> Caret {
        let mut char_sum = 0;

        for (i, line) in self.rope.lines().enumerate() {
            char_sum += line.len_chars();
            if char_sum > idx {
                let caret_ln = i;
                let caret_col = idx - (char_sum - line.len_chars());
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
        
        self.rope.len_chars() - 1

    }

    pub fn set_caret_position(&mut self, line: usize, column: usize, selection: bool) {
        let old_idx = self.char_idx;
        self.char_idx = self.get_char_idx(Caret::from(line, column));

        if selection {
            self.set_selection(true, old_idx);
        } else {
            self.clear_selection();
        }
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
        };

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

    pub fn clear_selection(&mut self) {
        self.selection = None;
    }

    pub fn set_selection(&mut self, selection: bool, old_idx: usize) {
        match (selection, self.selection) {
            (true, Some((_start, end))) if self.char_idx == end => {
                self.selection = None;
            }

            (true, None) => {
                self.selection = Some((old_idx, self.char_idx));
            }

            (true, Some((start, _end))) => {
                self.selection = Some((start, self.char_idx));
            }

            (false, _) => {
                self.selection = None;
            }
        }
    }


    pub fn backspace(&mut self) {
        if self.selection.is_some() {
            self.delete_selection();
            return;
        }

        if self.char_idx == 0 {
            return;
        }

        self.apply_new_event(Event::RemoveChar(self.rope.char(self.char_idx - 1), self.char_idx - 1));
    }

    pub fn delete(&mut self) {
        if self.selection.is_some() {
            self.delete_selection();
            return;
        }

        if self.char_idx == self.rope.len_chars() - 1 {
            return;
        }

        self.apply_new_event(Event::RemoveChar(self.rope.char(self.char_idx), self.char_idx));
    }

    pub fn insert_char(&mut self, c: char) {
        self.delete_selection();

        self.apply_new_event(Event::AddChar(c, self.char_idx));
    }

    pub fn insert_newline(&mut self) {
        self.insert_char('\n');
    }

    pub fn insert_string(&mut self, s: String) {
        self.delete_selection();

        self.apply_new_event(Event::AddString(s, self.char_idx));
    }

    pub fn get_selection(&self) -> Option<String> {
        match self.selection {
            Some((start, end)) => {
                let s = min(start, end);
                let e = max(start, end);
                Some(self.rope.slice(s..=e).to_string())
            }
            None => None,
        }
    }

    pub fn apply_new_event(&mut self, event: Event) {
        self.event_history.truncate(self.history_idx);

        self.event_history.push(event.clone());

        self.apply_event(event);

        self.history_idx += 1;

        self.dirty_changes = Some(self.dirty_changes.map(|d| d + 1).unwrap_or(1));
    }

    pub fn delete_selection(&mut self) {
        if let Some((start, end)) = self.selection {

            let s = min(start, end);
            let e = max(start, end); 

            self.apply_new_event(Event::RemoveString(self.rope.slice(s..e).to_string(), s));

            self.clear_selection();
        }
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
            }
            Event::AddString(s,idx) => {
                self.rope.insert(idx, &s);
                let new_idx = idx + s.len();
                self.char_idx = new_idx;
            }
            Event::RemoveString(s, idx) => {
                self.rope.remove(idx..idx + s.len());
                self.char_idx = idx;
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
            }
            Event::RemoveChar(c, idx) => {
                self.rope.insert_char(idx, c);
                self.char_idx = idx;
                self.caret_move_right();
            }
            Event::AddString(s, idx) => {
                self.rope.remove(idx..idx + s.len());
                self.char_idx = idx;
            }
            Event::RemoveString(s, idx ) => {
                self.rope.insert(idx, &s);
                let new_idx = idx + s.len();
                self.char_idx = new_idx;
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
