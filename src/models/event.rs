use super::text::Caret;

pub enum Event {
    AddChar(char, Caret),
    RemoveChar(char, Caret),
    AddNewLine(Caret),
}