
#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    AddChar(char, usize),
    RemoveChar(char, usize),

    AddString(String, usize),
    RemoveString(String, usize),
}