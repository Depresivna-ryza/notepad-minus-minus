use dioxus::{
    hooks::use_signal,
    signals::{ReadOnlySignal, Signal},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ShownPanels {
    pub terminal: Signal<bool>,
    pub search: Signal<bool>,
    pub file_tree: Signal<bool>,
    pub sessions: Signal<bool>,
    pub history: Signal<bool>,
}

pub struct ReadOnlyShownPanels {
    pub terminal: ReadOnlySignal<bool>,
    pub search: ReadOnlySignal<bool>,
    pub file_tree: ReadOnlySignal<bool>,
    pub sessions: ReadOnlySignal<bool>,
    pub history: ReadOnlySignal<bool>,
}

impl From<ShownPanels> for ReadOnlyShownPanels {
    fn from(panels: ShownPanels) -> Self {
        Self {
            terminal: panels.terminal.into(),
            search: panels.search.into(),
            file_tree: panels.file_tree.into(),
            sessions: panels.sessions.into(),
            history: panels.history.into(),
        }
    }
}

impl Default for ShownPanels {
    fn default() -> Self {
        Self::new()
    }
}

impl ShownPanels {
    pub fn new() -> Self {
        Self {
            terminal: use_signal(|| false),
            search: use_signal(|| false),
            file_tree: use_signal(|| true),
            sessions: use_signal(|| false),
            history: use_signal(|| false),
        }
    }
}
