use std::{cmp::{max, min}, rc::Rc};

use crate::models::{tabs::Tabs, text::{Caret, TextFile}};

use arboard::Clipboard;
use dioxus::prelude::*;
use itertools::Itertools;
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

            onkeydown: move |e| {
                
                let ctrl = e.modifiers().contains(Modifiers::CONTROL);
                let shift = e.modifiers().contains(Modifiers::SHIFT);

                info!("key pressed: {:?}, ctrl: {}, shift: {}", e.key(), ctrl, shift);

                let mut tabss = tabs.write();
                let Some(ref mut file) = tabss.get_current_file_mut() else {
                    return;
                };

                let old_idx = file.char_idx;

                match (e.key(), ctrl, shift) {
                    (Key::End, ctrl, selection) => {
                        file.caret_move_line_end(ctrl);
                        file.set_selection(selection, old_idx);
                    }
                    (Key::Home, ctrl, selection) => {
                        file.caret_move_line_start(ctrl);
                        file.set_selection(selection, old_idx);
                    }
                    (Key::ArrowLeft, ctrl, selection) => {
                        file.caret_move_left(ctrl);
                        file.set_selection(selection, old_idx);
                    }
                    (Key::ArrowRight, ctrl, selection) => {
                        file.caret_move_right(ctrl);
                        file.set_selection(selection, old_idx);
                    }
                    (Key::ArrowUp, false, selection) => {
                        file.caret_move_up();
                        file.set_selection(selection, old_idx);
                    }
                    (Key::ArrowDown, false, selection) => {
                        file.caret_move_down();
                        file.set_selection(selection, old_idx);
                    }

                    (Key::Character(s), false, _) => {
                        if let Some(c) = s.chars().next() {
                            info!("inserting char: {:?}", c);
                            file.insert_char(c);
                        }
                    }
                    
                    (Key::Backspace, ctrl, false) => {
                        file.backspace(ctrl);
                    }
                    
                    (Key::Delete, ctrl, false) => {
                        file.delete(ctrl);
                    }

                    (Key::Escape, false, _) => {
                        file.clear_selection();
                    }

                    (Key::Enter, false, _) => {
                        file.clear_selection();
                        file.insert_newline();
                    }

                    (Key::Tab, false, _) => {
                        file.clear_selection();
                        file.insert_tab();
                        e.prevent_default();
                    }

                    (Key::Character(z), true, false) if &z.to_ascii_lowercase() == "z" => {
                        info!("undo pressed");
                        file.clear_selection();
                        file.undo_event();
                    }

                    (Key::Character(z), true, true) if &z.to_ascii_lowercase() == "z" => {
                        info!("redo pressed");
                        file.clear_selection();
                        file.redo_event();
                    }

                    (Key::Character(s), true, false) if &s.to_ascii_lowercase() == "s" => {
                        info!("save pressed");
                        file.clear_selection();
                        file.save_to_file();
                    }

                    (Key::Character(x), true, false) if &x.to_ascii_lowercase() == "x" => {
                        info!("cut pressed");
                        file.cut_line();
                    }

                    (Key::Character(c), true, false) if &c.to_ascii_lowercase() == "c" => {
                        info!("copy pressed");
                        if let Some(selection) = file.get_selection() {
                            let mut clipboard = Clipboard::new().ok();
                            if let Some(clip) = clipboard.as_mut() {
                                if let Err(_) = clip.set_text(selection.clone()) {
                                    info!("failed to copy to clipboard");
                                } else {
                                    info!("copied to clipboard: {:?}", selection);
                                }
                            }
                        }
                    }

                    (Key::Character(v), true, false) if &v.to_ascii_lowercase() == "v" => {
                        info!("paste pressed");
                        let mut clipboard = Clipboard::new().ok();
                        if let Some(clip) = clipboard.as_mut() {
                            if let Ok(text) = clip.get_text() {
                                file.insert_string(text);
                            }
                        }
                    }

                    

                    (_,_,_) => {}
                }

            },

            style: "display: flex; flex-direction: column; flex: 1; justify-content: space-between; height: 10px;",
            TopStatusBar {tabs},
            EditorText {tabs, 
                // text,
                 caret_col: caret_col(), caret_line: caret_line()},
            BottomStatusBar {tabs, caret_col: caret_col(), caret_line: caret_line(), char_idx: text.read().clone().map_or(0, |t| t.char_idx)},
        }
    }
}

#[component]
pub fn EditorText(
    tabs: Signal<Tabs>,
    // text: Memo<Option<TextFile>>,
    caret_col: usize,
    caret_line: usize,
) -> Element {
    let Some(ref text) = tabs.read().get_current_file() else {
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

            style: "background-color: purple; display: flex; overflow-y: scroll; flex: 1; flex-direction: column",
            for (i, line) in text.chars().into_iter().enumerate() {
                
                EditorLine {
                    tabs: tabs,
                    selection_start: tabs.read().get_current_file().map(|f| f.selection.map(|s| f.get_caret_from_idx(min(s.0, s.1)))).flatten(),
                    selection_end: tabs.read().get_current_file().map(|f| f.selection.map(|s| f.get_caret_from_idx(max(s.0, s.1)))).flatten(),
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
    selection_start: Option<Caret>,
    selection_end: Option<Caret>,
    content: String,
    line_i: usize,
    caret_col: usize,
    caret_line: usize,
    parent_element: Signal<Option<Rc<MountedData>>>,
) -> Element {
    
    let mut element: Signal<Option<Rc<MountedData>>> = use_signal(|| None);

    let _ = use_resource(move || async move {
        if line_i == caret_line {
            if let Some(ref elem) = *element.read() {
                if let Some(ref parent_elem) = *parent_element.read() {
                    let scroll_offset = parent_elem.get_scroll_offset().await.unwrap();
                    let scroll_size = parent_elem.get_scroll_size().await.unwrap();
                    let parent_rect = parent_elem.get_client_rect().await.unwrap();

                    let client_rect = elem.get_client_rect().await.unwrap();

                    // dbg!(parent_rect.min_y(), parent_rect.max_y());
                    // dbg!(client_rect.min_y(), scroll_offset.y);
                    // dbg!(client_rect.max_y(), scroll_offset.y + scroll_size.height);
        
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

            style: match line_i == caret_line {
                true => "display: flex; flex-direction: row; background-color: gray;",
                false => "display: flex; flex-direction: row;"
            }.to_string() + "font-family: monospace; font-size: 16px; white-space: pre ",
            
            span {
                style: "padding-right: 10px; min-width: 30px; background-color: darkblue;",
                "{line_i}"
            }

            for (i, c) in content.chars().into_iter().map(|c| if c.clone() != '\n' { c.clone()} else {' '}).enumerate() {
                span {
                    onclick: move |e| {
                        info!("clicked on line: {:?}, col: {:?}, char: {}", line_i, i, c);
                        let selection = e.modifiers().contains(Modifiers::SHIFT);
                        tabs.write().get_current_file_mut().map(|file| file.set_caret_position(line_i, i, selection));
                    },

                    style:
                        if i == caret_col && line_i == caret_line {
                            "; background-color: yellow;"
                        } else if let (Some(start), Some(end)) = (selection_start, selection_end) {
                            if (start.ln < line_i && line_i < end.ln ) || 
                                (start.ln == line_i && line_i == end.ln && start.col <= i && i <= end.col) ||
                                (start.ln == line_i && line_i < end.ln && start.col <= i) ||
                                (start.ln < line_i && line_i == end.ln && i <= end.col) {
                                "; background-color: lightblue;"
                            } else {
                                ""
                            }
                        } else {
                            ""
                        },
                    "{c}"
                }
            }

            span {
                style: "flex: 1;",
                onclick: move |e| {
                    info!("clicked on line: {:?}", line_i);
                    let selection = e.modifiers().contains(Modifiers::SHIFT);
                    tabs.write().get_current_file_mut().map(|file| file.set_caret_position(line_i, content.len() - 1, selection));
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
    caret_col: usize,
    caret_line: usize,
    char_idx: usize,
) -> Element {
    let status = if let Some(ref f) = tabs.read().get_current_file() {
        if let Some((start, end)) = f.selection {
            let s = min(start, end);
            let e = max(start, end);
            let len = e - s;
            let words = f.rope.slice(s..=e).chars().tuple_windows().filter(|(a, b)| a.is_whitespace() && !b.is_whitespace()).count() + 1;

            format!("Selection: {len} chars, {words} words")
        } else if let Some(x) = f.dirty_changes {
            format!("{x} unsaved changes")
        } else {
            format!("(no changes)")
        }
    } else {
        String::new()
    };

    rsx! {
        div {
            style: "background-color: blue; height: 30px; display: flex; justify-content: flex-end; align-items: center;",
            span {
                style: "margin-right: 10px;",
                "{status}"
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
