use std::rc::Rc;

use crate::models::{tabs::Tabs, text::{Caret, TextFile}};

use dioxus::prelude::*;
use dioxus_elements::{geometry::{euclid::{Size2D, Vector2D}, Pixels}, span};
use tracing::info;

#[component]
pub fn Editor(tabs: Signal<Tabs>) -> Element {
    let text: Memo<Option<TextFile>> = use_memo(move || tabs.read().get_current_file());

    let caret = use_memo(move || match text.read().clone() {
        Some(text) => text.get_caret(),
        None => Caret::new(),
    });

    let caret_col = use_memo(move || caret.read().col);

    let caret_line = use_memo(move || caret.read().ln);

    rsx! {
        div {
            tabindex: 0,

            onfocusin: move |e| {
                info!("focused on editor: {:?}", e);
            },

            onfocusout: move |e| {
                info!("unfocused on editor: {:?}", e);
            },

            onkeyup: move |e| {
                
                let ctrl = e.modifiers().contains(Modifiers::CONTROL);
                let shift = e.modifiers().contains(Modifiers::SHIFT);

                info!("key pressed: {:?}, ctrl: {}, shift: {}", e.key(), ctrl, shift);

                match (e.key(), ctrl, shift) {
                    (Key::ArrowLeft, false, _) => {
                        tabs.write().get_current_file_mut().map(|file| file.caret_move_left());
                    }
                    (Key::ArrowRight, false, _) => {
                        tabs.write().get_current_file_mut().map(|file| file.caret_move_right());
                    }
                    (Key::ArrowUp, false, _) => {
                        tabs.write().get_current_file_mut().map(|file| file.caret_move_up());
                    }
                    (Key::ArrowDown, false, _) => {
                        tabs.write().get_current_file_mut().map(|file| file.caret_move_down());
                    }

                    (Key::Character(s), false, _) => {
                        if let Some(c) = s.chars().next(){
                            info!("inserting char: {:?}", c);
                            tabs.write().get_current_file_mut().map(|file| file.insert_char(c));
                        }
                    }

                    (Key::Backspace, false, _) => {
                        tabs.write().get_current_file_mut().map(|file| file.backspace());
                    }

                    (Key::Delete, false, _) => {
                        tabs.write().get_current_file_mut().map(|file| file.delete());
                    }

                    (Key::Enter, false, _) => {
                        tabs.write().get_current_file_mut().map(|file| file.insert_newline());
                    }

                    (Key::Character(z), true, false) if &z.to_ascii_lowercase() == "z" => {
                        info!("undo pressed");
                        tabs.write().get_current_file_mut().map(|file| file.undo_event());
                    }

                    (Key::Character(z), true, true) if &z.to_ascii_lowercase() == "z" => {
                        info!("redo pressed");
                        tabs.write().get_current_file_mut().map(|file| file.redo_event());
                    }

                    (Key::Character(s), true, false) if &s.to_ascii_lowercase() == "s" => {
                        info!("save pressed");
                        tabs.write().get_current_file_mut().map(|file| file.save_to_file());
                    }

                    (_,_,_) => {}
                }

            },

            style: "display: flex; flex-direction: column; flex: 1; justify-content: space-between; height: 10px;",
            TopStatusBar {tabs},
            EditorText {tabs, text, caret_col: caret_col(), caret_line: caret_line()},
            BottomStatusBar {tabs, caret_col: caret_col(), caret_line: caret_line(), char_idx: text.read().clone().map_or(0, |t| t.char_idx)},
        }
    }
}

#[component]
pub fn EditorText(
    tabs: Signal<Tabs>,
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

    let mut element: Signal<Option<Rc<MountedData>>> = use_signal(|| None);

    rsx! {
        div {
            onmounted: move |e| {
                info!("mounted line: {:?}", e);
                element.set(Some(e.data()));
            },

            style: "background-color: purple; flex: 1; overflow-y: auto; flex: 1;",
            for (i, line) in text.chars().into_iter().enumerate() {
                
                EditorLine {
                    tabs: tabs,
                    content: line, 
                    line_i: i, 
                    caret_col: caret_col, 
                    caret_line: caret_line,
                    parent_element: element,
                }
            }
        }
    }
}

#[component]
pub fn EditorLine(
    tabs: Signal<Tabs>,
    content: ReadOnlySignal<Vec<char>>, 
    line_i: ReadOnlySignal<usize>,
    caret_col: ReadOnlySignal<usize>,
    caret_line: ReadOnlySignal<usize>,
    parent_element: Signal<Option<Rc<MountedData>>>,
) -> Element {
    
    let mut element: Signal<Option<Rc<MountedData>>> = use_signal(|| None);

    let _ = use_resource(move || async move {
        if line_i == caret_line() {
            if let Some(ref elem) = *element.read() {
                if let Some(ref parent_elem) = *parent_element.read() {
                    let scroll_offset = parent_elem.get_scroll_offset().await.unwrap();
                    let scroll_size = parent_elem.get_scroll_size().await.unwrap();
                    let parent_rect = parent_elem.get_client_rect().await.unwrap();

                    let client_rect = elem.get_client_rect().await.unwrap();

                    dbg!(parent_rect.min_y(), parent_rect.max_y());
                    dbg!(client_rect.min_y(), scroll_offset.y);
                    dbg!(client_rect.max_y(), scroll_offset.y + scroll_size.height);
        
                    let height_underflown = client_rect.min_y() < parent_rect.min_y();
                    let height_overflown = client_rect.max_y() > parent_rect.max_y();
        
                    if height_underflown || height_overflown {
                        let _ = elem.scroll_to(ScrollBehavior::Instant).await;
                    }
                }
            }
        }
    });


    rsx! {
        div {
            onmounted: move |e| {
                info!("mounted line: {:?}", e);
                element.set(Some(e.data()));
            },

            style: match line_i == caret_line() {
                true => "display: flex; flex-direction: row; background-color: gray;",
                false => "display: flex; flex-direction: row;"
            },
            
            span {
                style: "padding-right: 10px; min-width: 30px; background-color: darkblue;",
                "{line_i}"
            }

            for (i, c) in content.iter().map(|c| if c.clone() != '\n' { c.clone()} else {' '}).enumerate() {
                span {
                    onclick: move |_| {
                        info!("clicked on line: {:?}, col: {:?}, char: {}", line_i, i, c);
                        tabs.write().get_current_file_mut().map(|file| file.set_caret_position(line_i(), i));
                    },
            
                    style: match (i == caret_col() && line_i == caret_line(), line_i == caret_line()) {
                        (true, true) => "font-family: monospace; background-color: yellow; font-size: 16px; white-space: pre",
                        _ => "font-family: monospace; font-size: 16px; white-space: pre"
                    },
                    "{c}"
                }
            }

            span {
                style: "flex: 1;",
                onclick: move |_| {
                    info!("clicked on line: {:?}", line_i);
                    info!("{:?}", content);
                    tabs.write().get_current_file_mut().map(|file| file.set_caret_position(line_i(), content().len() - 1));
                }
            }
        }
    }
}

#[component]
pub fn TopStatusBar(tabs: Signal<Tabs>) -> Element {
    let path: Option<Vec<String>> = tabs()
        .current_file
        .map(|p| p.iter().map(|p| p.to_string_lossy().to_string()).collect());
   
    rsx! {
        div {
            style: "background-color: blue; height: 40px; display: flex; justify-content: space-between; align-items: center; ",
            Breadcrumbs {path},
            button {
                onclick: move |_| {
                    tabs.write().get_current_file_mut().map(|file| file.save_to_file());
                },

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
    char_idx: ReadOnlySignal<usize>,
) -> Element {
    rsx! {
        div {
            style: "background-color: blue; height: 30px; display: flex; justify-content: flex-end; align-items: center;",
            span {
                style: "margin-right: 10px;",
                if let Some(ref f) = tabs.read().get_current_file() {
                    if let Some(x) = f.dirty_changes {
                        "{x} unsaved changes"
                    } else {
                        "(no changes)"
                    }
                } else {
                    ""
                }
            }
            
            div {
                style: "flex: 1;",
            }

            span {
                style: "margin-left: 10px;",
                "Line: {caret_line}, Col: {caret_col} | Char: {char_idx}"
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
