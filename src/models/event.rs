
#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    AddChar(char, usize),
    RemoveChar(char, usize),
}