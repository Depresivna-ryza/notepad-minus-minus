use std::{
    cmp::{max, min},
    fs::read_to_string,
    path::PathBuf,
};

use super::historyevent::HistoryEvent;
use ropey::Rope;

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Caret {
    pub ln: usize,
    pub col: usize,
}

impl Default for Caret {
    fn default() -> Self {
        Self::new()
    }
}

impl Caret {
    pub fn from(l: usize, c: usize) -> Self {
        Self { ln: l, col: c }
    }
    pub fn new() -> Self {
        Self { ln: 0, col: 0 }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TextFile {
    pub path: PathBuf,
    pub rope: Rope,
    pub char_idx: usize,

    pub event_history: Vec<HistoryEvent>,
    pub history_idx: usize,
    pub dirty_changes: Option<usize>,

    pub selection: Option<(usize, usize)>,
}

impl TextFile {
    pub fn new(path: PathBuf) -> Self {
        let mut content = read_to_string(&path)
            .ok()
            .unwrap_or_default()
            .replace("\r\n", "\n");

        if !content.ends_with('\n') {
            content.push('\n');
        }

        Self {
            path,
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
            Err(_e) => {}
        }
    }

    pub fn to_string(&self) -> String {
        self.rope.to_string()
    }

    pub fn chars(&self) -> Vec<String> {
        self.rope
            .lines()
            .filter_map(|line| {
                if line.len_chars() == 0 {
                    None
                } else {
                    Some(line.to_string())
                }
            })
            .collect()
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

        Caret::from(
            self.rope.len_lines() - 1,
            self.rope.line(self.rope.len_lines() - 1).len_chars(),
        )
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

        Caret::from(
            self.rope.len_lines() - 1,
            self.rope.line(self.rope.len_lines() - 1).len_chars(),
        )
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

    pub fn caret_move_left(&mut self, ctrl: bool) {
        match (ctrl, self.char_idx) {
            (_, 0) => {
                self.char_idx = 0;
            }

            (false, i) => {
                self.char_idx = i - 1;
            }

            (true, i) => {
                // let mut new_char_idx = i - 1;

                // let mut skipped_whitespace = !self.rope.char(new_char_idx).is_ascii_whitespace();

                // while new_char_idx > 0 &&
                //     self.rope.char(new_char_idx - 1) != '\n' &&
                //     (self.rope.char(new_char_idx).is_ascii_alphanumeric() || !skipped_whitespace) &&
                //     (self.rope.char(new_char_idx).is_ascii_whitespace() || skipped_whitespace) {

                //     new_char_idx -= 1;

                //     if !self.rope.char(new_char_idx).is_ascii_whitespace() {
                //         skipped_whitespace = true;
                //     }
                // }

                // self.char_idx = new_char_idx;

                self.char_idx = i - 1;
                self.char_idx = self.ctrl_idx_move(true);
            }
        }
    }

    pub fn caret_move_right(&mut self, ctrl: bool) {
        match (ctrl, self.char_idx) {
            (_, i) if i + 1 >= self.rope.len_chars() => {
                self.char_idx = self.rope.len_chars() - 1;
            }

            (false, i) => {
                self.char_idx = i + 1;
            }

            (true, i) => {
                self.char_idx = i + 1;
                self.char_idx = self.ctrl_idx_move(false);
            }
        }
    }

    fn ctrl_idx_move(&self, go_left: bool) -> usize {
        let mut idx = self.char_idx;

        let mv = |i: usize| match (go_left, i) {
            (true, 0) => 0,
            (false, i) if i + 1 >= self.rope.len_chars() => self.rope.len_chars() - 1,
            (true, i) => i - 1,
            (false, i) => i + 1,
        };

        let mut skipped_whitespace = !self.rope.char(idx).is_ascii_whitespace();

        while idx != mv(idx)
            && self.rope.char(idx) != '\n'
            && (self.rope.char(idx).is_ascii_alphanumeric() || !skipped_whitespace)
            && (self.rope.char(idx).is_ascii_whitespace() || skipped_whitespace)
            && (!go_left || !skipped_whitespace || !self.rope.char(mv(idx)).is_ascii_whitespace())
        {
            idx = mv(idx);

            if !self.rope.char(idx).is_ascii_whitespace() {
                skipped_whitespace = true;
            }
        }

        idx
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
            self.char_idx =
                self.char_idx - caret.col + line.len_chars() + next_line.len_chars() - 1;
        }
    }

    pub fn caret_move_up(&mut self) {
        let caret = self.get_caret();

        if caret.ln == 0 {
            self.char_idx = 0;
            return;
        }

        let Some(_line) = self.rope.get_line(caret.ln) else {
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

    pub fn caret_move_line_end(&mut self, ctrl: bool) {
        if ctrl {
            self.char_idx = self.rope.len_chars() - 1;
            return;
        }

        let mut new_char_idx = self.char_idx;

        while new_char_idx < self.rope.len_chars() - 1 && self.rope.char(new_char_idx) != '\n' {
            new_char_idx += 1;
        }

        self.char_idx = new_char_idx;
    }

    pub fn caret_move_line_start(&mut self, ctrl: bool) {
        if ctrl {
            self.char_idx = 0;
            return;
        }

        let mut new_char_idx = self.char_idx;

        while new_char_idx > 0 && self.rope.char(new_char_idx - 1) != '\n' {
            new_char_idx -= 1;
        }

        self.char_idx = new_char_idx;
    }

    pub fn clear_selection(&mut self) {
        self.selection = None;
    }

    pub fn set_selection(&mut self, selection: bool, old_idx: usize) {
        match (selection, self.selection) {
            (true, Some((start, _end))) if self.char_idx == start => {
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

    pub fn backspace(&mut self, ctrl: bool) {
        if self.selection.is_some() {
            self.delete_selection();
            return;
        }

        if self.char_idx == 0 {
            return;
        }

        if !ctrl {
            self.apply_new_event(HistoryEvent::RemoveChar(
                self.rope.char(self.char_idx - 1),
                self.char_idx - 1,
            ));
            return;
        }

        let end_of_deletion_idx = self.char_idx;
        self.char_idx -= 1;
        let start_of_deletion_idx = self.ctrl_idx_move(true);

        self.apply_new_event(HistoryEvent::RemoveString(
            self.rope
                .slice(start_of_deletion_idx..end_of_deletion_idx)
                .to_string(),
            start_of_deletion_idx,
        ));
    }

    pub fn delete(&mut self, ctrl: bool) {
        if self.selection.is_some() {
            self.delete_selection();
            return;
        }

        if self.char_idx == self.rope.len_chars() - 1 {
            return;
        }

        if !ctrl {
            self.apply_new_event(HistoryEvent::RemoveChar(
                self.rope.char(self.char_idx),
                self.char_idx,
            ));
            return;
        }

        let start_of_deletion_idx = self.char_idx;
        self.char_idx += 1;
        let end_of_deletion_idx = self.ctrl_idx_move(false);

        self.apply_new_event(HistoryEvent::RemoveString(
            self.rope
                .slice(start_of_deletion_idx..end_of_deletion_idx)
                .to_string(),
            start_of_deletion_idx,
        ));
    }

    pub fn cut_line(&mut self) -> String {
        let start_idx = self.rope.line_to_char(self.get_caret().ln);
        let end_idx = self.rope.line_to_char(self.get_caret().ln + 1);
        let res = self.rope.slice(start_idx..end_idx).to_string();

        self.apply_new_event(HistoryEvent::RemoveString(
            self.rope.slice(start_idx..end_idx).to_string(),
            start_idx,
        ));

        res
    }

    pub fn move_line(&mut self, go_down: bool) {
        self.apply_new_event(HistoryEvent::MoveLine(self.get_caret().ln, go_down));
    }

    pub fn duplicate_line(&mut self, go_down: bool) {
        let start_idx = self.rope.line_to_char(self.get_caret().ln);
        let end_idx = self.rope.line_to_char(self.get_caret().ln + 1);
        let line_offset = self.char_idx - start_idx;

        let chars = self.rope.slice(start_idx..end_idx).chars();

        let shifted_line = chars
            .cycle()
            .skip(line_offset)
            .take(end_idx - start_idx)
            .collect::<String>();

        self.apply_new_event(HistoryEvent::AddString(shifted_line, self.char_idx));

        if !go_down {
            self.char_idx -= end_idx - start_idx;
        }
    }

    pub fn insert_char(&mut self, c: char) {
        self.delete_selection();

        self.apply_new_event(HistoryEvent::AddChar(c, self.char_idx));
    }

    pub fn insert_newline(&mut self) {
        self.insert_char('\n');
    }

    pub fn insert_tab(&mut self) {
        self.insert_string("    ".to_string());
    }

    pub fn insert_string(&mut self, s: String) {
        self.delete_selection();

        self.apply_new_event(HistoryEvent::AddString(s, self.char_idx));
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

    pub fn find_and_select(
        &mut self,
        start: usize,
        mut needle: String,
        reverse: bool,
        case_sensitive: bool,
    ) -> Option<usize> {
        let mut s = match reverse {
            true => self
                .rope
                .slice(..=start)
                .to_string()
                .chars()
                .rev()
                .collect(),
            false => self.rope.slice(start..).to_string(),
        };

        if !case_sensitive {
            s = s.to_lowercase();
            needle = needle.to_lowercase();
        }

        if reverse {
            needle = needle.chars().rev().collect();
        }

        let idx = s.find(&needle);

        match (idx, reverse) {
            (Some(i), false) => {
                let start_idx = start + i;
                let end_idx = start_idx + needle.len();
                self.selection = Some((start_idx, end_idx - 1));
                self.char_idx = start_idx;
                Some(start_idx)
            }

            (Some(i), true) => {
                let start_idx = start + 1 - i - needle.len();
                let end_idx = start_idx + needle.len();

                self.selection = Some((start_idx, end_idx - 1));
                self.char_idx = start_idx;
                Some(start_idx)
            }

            (None, _) => {
                self.clear_selection();
                None
            }
        }
    }

    pub fn apply_new_event(&mut self, event: HistoryEvent) {
        self.event_history.truncate(self.history_idx);

        self.event_history.push(event.clone());

        self.apply_event(event);

        self.history_idx += 1;

        self.dirty_changes = Some(self.dirty_changes.map(|d| d + 1).unwrap_or(1));

        self.ammend_history();
    }

    pub fn ammend_history(&mut self) {
        if self.history_idx != self.event_history.len() || self.event_history.len() < 2 {
            return;
        }

        let Some(new) = self.event_history.last().cloned() else {
            return;
        };

        let Some(last) = self.event_history.get(self.history_idx - 2).cloned() else {
            return;
        };

        let new_event: HistoryEvent;

        match (last.clone(), new.clone()) {
            (HistoryEvent::AddChar(c1, idx1), HistoryEvent::AddChar(c2, idx2))
                if idx1 + 1 == idx2 && c1.is_ascii_whitespace() == c2.is_ascii_whitespace() =>
            {
                // self.event_history.pop();
                // self.event_history.pop();

                // self.event_history.push(HistoryEvent::AddString(format!("{}{}", c1, c2), idx1));

                new_event = HistoryEvent::AddString(format!("{}{}", c1, c2), idx1);
            }

            (HistoryEvent::AddString(s1, idx1), HistoryEvent::AddChar(c2, idx2))
                if idx1 + s1.len() == idx2
                    && ((c2.is_ascii_whitespace()
                        && s1.chars().all(|c| c.is_ascii_whitespace()))
                        || (!c2.is_ascii_whitespace()
                            && s1.chars().all(|c| !c.is_ascii_whitespace()))) =>
            {
                new_event = HistoryEvent::AddString(format!("{}{}", s1, c2), idx1);
            }

            (HistoryEvent::RemoveChar(c1, idx1), HistoryEvent::RemoveChar(c2, idx2))
                if idx1 == idx2 + 1 && c1.is_ascii_whitespace() == c2.is_ascii_whitespace() =>
            {
                new_event = HistoryEvent::RemoveString(format!("{}{}", c2, c1), idx2);
            }

            (HistoryEvent::RemoveChar(c1, idx1), HistoryEvent::RemoveChar(c2, idx2))
                if idx1 == idx2 && c1.is_ascii_whitespace() == c2.is_ascii_whitespace() =>
            {
                new_event = HistoryEvent::RemoveString(format!("{}{}", c1, c2), idx2);
            }

            (HistoryEvent::RemoveString(s1, idx1), HistoryEvent::RemoveChar(c2, idx2))
                if idx1 == idx2 + 1
                    && ((c2.is_ascii_whitespace()
                        && s1.chars().all(|c| c.is_ascii_whitespace()))
                        || (!c2.is_ascii_whitespace()
                            && s1.chars().all(|c| !c.is_ascii_whitespace()))) =>
            {
                new_event = HistoryEvent::RemoveString(format!("{}{}", c2, s1), idx2);
            }

            (HistoryEvent::RemoveString(s1, idx1), HistoryEvent::RemoveChar(c2, idx2))
                if idx1 == idx2
                    && ((c2.is_ascii_whitespace()
                        && s1.chars().all(|c| c.is_ascii_whitespace()))
                        || (!c2.is_ascii_whitespace()
                            && s1.chars().all(|c| !c.is_ascii_whitespace()))) =>
            {
                new_event = HistoryEvent::RemoveString(format!("{}{}", s1, c2), idx2);
            }

            _ => return,
        }

        self.event_history.pop();
        self.event_history.pop();
        self.event_history.push(new_event);

        self.history_idx -= 1;

        self.dirty_changes = Some(
            self.dirty_changes
                .and_then(|d| d.checked_sub(1))
                .unwrap_or(0),
        );
    }

    pub fn delete_selection(&mut self) {
        if let Some((start, end)) = self.selection {
            let s = min(start, end);
            let e = max(start, end);

            self.apply_new_event(HistoryEvent::RemoveString(
                self.rope.slice(s..e).to_string(),
                s,
            ));

            self.clear_selection();
        }
    }

    pub fn apply_event(&mut self, event: HistoryEvent) {
        match event {
            HistoryEvent::AddChar(c, idx) => {
                self.rope.insert_char(idx, c);
                self.char_idx = idx;
                self.caret_move_right(false);
            }
            HistoryEvent::RemoveChar(_c, idx) => {
                self.rope.remove(idx..idx + 1);
                self.char_idx = idx;
            }
            HistoryEvent::AddString(s, idx) => {
                self.rope.insert(idx, &s);
                let new_idx = idx + s.len();
                self.char_idx = new_idx;
            }
            HistoryEvent::RemoveString(s, idx) => {
                self.rope.remove(idx..idx + s.len());
                self.char_idx = idx;
            }

            HistoryEvent::MoveLine(ln, go_down) => {
                let remove_caret_line = match (go_down, ln) {
                    (true, i) if i + 1 >= self.rope.len_lines() => return,
                    (false, 0) => return,
                    (true, i) => i,
                    (false, i) => i - 1,
                };

                let line_0 = self.rope.line_to_char(remove_caret_line);
                let line_1 = self.rope.line_to_char(remove_caret_line + 1);
                let line_2 = self.rope.line_to_char(remove_caret_line + 2);

                let removed_line_content = self.rope.slice(line_0..line_1).to_string();

                self.rope.remove(line_0..line_1);
                self.rope
                    .insert(line_2 - (line_1 - line_0), &removed_line_content);

                self.char_idx = self
                    .rope
                    .line_to_char(if go_down { ln + 1 } else { ln - 1 });
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
            HistoryEvent::AddChar(_c, idx) => {
                self.rope.remove(idx..idx + 1);
                self.char_idx = idx;
            }
            HistoryEvent::RemoveChar(c, idx) => {
                self.rope.insert_char(idx, c);
                self.char_idx = idx;
                self.caret_move_right(false);
            }
            HistoryEvent::AddString(s, idx) => {
                self.rope.remove(idx..idx + s.len());
                self.char_idx = idx;
            }
            HistoryEvent::RemoveString(s, idx) => {
                self.rope.insert(idx, &s);
                let new_idx = idx + s.len();
                self.char_idx = new_idx;
            }

            HistoryEvent::MoveLine(ln, go_down) => match go_down {
                true => self.apply_event(HistoryEvent::MoveLine(ln + 1, false)),
                false => self.apply_event(HistoryEvent::MoveLine(ln - 1, true)),
            },
        }

        self.history_idx -= 1;
        self.dirty_changes = Some(
            self.dirty_changes
                .and_then(|d| d.checked_sub(1))
                .unwrap_or(0),
        );
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

    pub fn go_to_history_idx(&mut self, idx: usize) {
        if idx > self.history_idx {
            for _ in 0..(idx - self.history_idx) {
                self.redo_event();
            }
        } else {
            for _ in 0..(self.history_idx - idx) {
                self.undo_event();
            }
        }
    }
}
