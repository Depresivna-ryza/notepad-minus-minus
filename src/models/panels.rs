use dioxus::{hooks::use_signal, signals::{ReadOnlySignal, Signal}};


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ShownPanels {
    pub terminal: Signal<bool>,
    pub search: Signal<bool>,
    pub file_tree: Signal<bool>,
    pub sessions: Signal<bool>,
}

pub struct ReadOnlyShownPanels {
    pub terminal: ReadOnlySignal<bool>,
    pub search: ReadOnlySignal<bool>,
    pub file_tree: ReadOnlySignal<bool>,
    pub sessions: ReadOnlySignal<bool>,
}

impl From<ShownPanels> for ReadOnlyShownPanels {
    fn from(panels: ShownPanels) -> Self {
        Self {
            terminal: panels.terminal.into(),
            search: panels.search.into(),
            file_tree: panels.file_tree.into(),
            sessions: panels.sessions.into(),
        }
    }
}

impl ShownPanels {
    pub fn new() -> Self {
        Self {
            terminal: use_signal(||false),
            search: use_signal(||false),
            file_tree: use_signal(||false),
            sessions: use_signal(||false),
        }
    }
}