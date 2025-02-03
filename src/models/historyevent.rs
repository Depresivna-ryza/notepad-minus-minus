use core::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub enum HistoryEvent {
    AddChar(char, usize),
    RemoveChar(char, usize),

    AddString(String, usize),
    RemoveString(String, usize),

    MoveLine(usize, bool),
}

impl Display for HistoryEvent {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let ws = |s: &str| match s {
            " " => String::from("_"),
            "\n" => String::from("⏎"),
            s if s.chars().all(char::is_whitespace) => s
                .chars()
                .map(|c| match c {
                    ' ' => '_',
                    '\n' => '⏎',
                    _ => c,
                })
                .collect(),
            s => s.to_string(),
        };

        match self {
            Self::AddChar(c, _) => write!(f, "Add `{}`", ws(&c.to_string())),
            Self::RemoveChar(c, _) => write!(f, "Del `{}`", ws(&c.to_string())),
            Self::AddString(s, _) => write!(f, "Add \"{}\"", ws(s)),
            Self::RemoveString(s, _) => write!(f, "Del \"{}\"", ws(s)),
            Self::MoveLine(i, b) => write!(f, "Move {} line {}", if *b { "up" } else { "down" }, i),
        }
    }
}
