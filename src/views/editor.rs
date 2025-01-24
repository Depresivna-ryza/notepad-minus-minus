use crate::models::{tabs::Tabs, text::TextFile};
use std::fs::read_to_string;

use dioxus::prelude::*;
use tracing::info;

#[component]
pub fn Editor(tabs: Signal<Tabs>) -> Element {
    let text: Memo<Option<TextFile>> = use_memo(move || tabs.read().get_current_file());

    let caret_col = use_memo(move || match text.read().clone() {
        Some(text) => text.caret_column,
        None => 0,
    });

    let caret_line = use_memo(move || match text.read().clone() {
        Some(text) => text.caret_line,
        None => 0,
    });

    rsx! {
        div {
            onfocusin: move |e| {
                info!("focused on editor: {:?}", e);
            },

            onfocusout: move |e| {
                info!("unfocused on editor: {:?}", e);
            },

            onclick: move |e| {
                info!("clicked on editor: {:?}", e);
            },

            onkeyup: move |e| {
                info!("key pressed: {:?}", e.key());

                match e.key() {
                    Key::ArrowLeft => {
                        tabs.write().get_current_file_mut().map(|file| file.caret_move_left());
                    }
                    Key::ArrowRight => {
                        tabs.write().get_current_file_mut().map(|file| file.caret_move_right());
                    }
                    Key::ArrowUp => {
                        tabs.write().get_current_file_mut().map(|file| file.caret_move_up());
                    }
                    Key::ArrowDown => {
                        tabs.write().get_current_file_mut().map(|file| file.caret_move_down());
                    }

                    Key::Character(s) => {
                        if let Some(c) = s.chars().next(){
                            info!("inserting char: {:?}", c);
                            tabs.write().get_current_file_mut().map(|file| file.insert_char(c));
                        }
                    }

                    Key::Delete | Key::Backspace => {
                        tabs.write().get_current_file_mut().map(|file| file.remove_char());
                    }

                    Key::Enter => {
                        tabs.write().get_current_file_mut().map(|file| file.insert_newline());
                    }

                    _ => {}
                }

            },

            style: "display: flex; flex-direction: column; flex: 1; justify-content: space-between; height: 10px;",
            TopStatusBar {tabs, text},
            EditorText {text, caret_col: caret_col(), caret_line: caret_line()},
            BottomStatusBar {tabs, caret_col: caret_col(), caret_line: caret_line()}
        }
    }
}

#[component]
pub fn EditorText(
    text: Memo<Option<TextFile>>,
    caret_col: ReadOnlySignal<usize>,
    caret_line: ReadOnlySignal<usize>,
) -> Element {
    let Some(ref text) = *text.read() else {
        return rsx! {
            div {
                style: "background-color: purple; flex: 1; color: red;",
                "<NO FILE>"
            }
        };
    };

    let lines = text.content.clone();
    rsx! {
        div {
            style: "background-color: purple; flex: 1; overflow-y: auto; flex: 1;",
            for (i, line) in lines.into_iter().enumerate() {
                EditorLine {content: line, line: i, caret_col: caret_col, caret_line: caret_line}
            }
        }
    }
}

#[component]
pub fn EditorLine(
    content: ReadOnlySignal<Vec<char>>, 
    line: ReadOnlySignal<usize>,
    caret_col: ReadOnlySignal<usize>,
    caret_line: ReadOnlySignal<usize>,
) -> Element {
    rsx! {
        div {
            style: match line == caret_line() {
                true => "display: flex; flex-direction: row; background-color: gray;",
                false => "display: flex; flex-direction: row;"
            },
            
            span {
                style: "padding-right: 10px; min-width: 30px; background-color: darkblue;",
                "{line}"
            }

            for (i, c) in content.iter().enumerate() {
                span {
                    style: match (i == caret_col() && line == caret_line(), line == caret_line()) {
                        (true, true) => "background-color: yellow;",
                        _ => ""
                    },
                    "{c}"
                }
            }

            if content.len() == caret_col() && line == caret_line(){
                span {
                    style: "background-color: yellow;",

                    "X"
                }
            }
        }
    }
}

#[component]
pub fn TopStatusBar(tabs: ReadOnlySignal<Tabs>, text: Memo<Option<TextFile>>) -> Element {
    let path: Option<Vec<String>> = tabs()
        .current_file
        .map(|p| p.iter().map(|p| p.to_string_lossy().to_string()).collect());

    rsx! {
        div {
            style: "background-color: blue; height: 40px; display: flex; justify-content: space-between; align-items: center; ",
            Breadcrumbs {path},
            button {
                style: "margin-right: 10px; flex-shrink: 0;",
                "Save"
            }
        }
    }
}

#[component]
pub fn BottomStatusBar(
    tabs: ReadOnlySignal<Tabs>,
    caret_col: ReadOnlySignal<usize>,
    caret_line: ReadOnlySignal<usize>,
) -> Element {
    rsx! {
        div {
            style: "background-color: blue; height: 30px; display: flex; justify-content: flex-end; align-items: center;",
            span {
                style: "margin-left: 10px;",
                "Line: {caret_line}, Col: {caret_col}"
            }
        }
    }
}

#[component]
pub fn Breadcrumbs(path: ReadOnlySignal<Option<Vec<String>>>) -> Element {
    let Some(path) = path() else {
        return rsx! {
            div {
                style: "background-color: green; height: 100%; display: flex; flex: 1; overflow-x: auto; white-space: nowrap;",
                "<No file selected>"
            }
        };
    };

    rsx! {
        div {
            class: "scrollbar-thin",
            style: "background-color: green; height: 100%; display: flex; flex: 1; overflow-x: auto; white-space: nowrap",
            for (i, part) in path.iter().enumerate() {
                span {
                    style: if i == path.len() - 1 {
                        "font-weight: bold;"
                    } else {
                        ""
                    },
                    "{part}"
                }

                if i < path.len() - 1 {
                    span {
                        style: "color: yellow; margin: 0 5px;",
                        ">"
                    }
                }
            }
        }
    }
}
