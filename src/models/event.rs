
#[derive(Debug, Clone, PartialEq)]
pub enum HistoryEvent {
    AddChar(char, usize),
    RemoveChar(char, usize),

    AddString(String, usize),
    RemoveString(String, usize),

    MoveLine(usize, bool)
}