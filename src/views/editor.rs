use std::rc::Rc;

use crate::models::{tabs::Tabs, text::TextFile};

use dioxus::prelude::*;
use dioxus_elements::geometry::{euclid::{Size2D, Vector2D}, Pixels};
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
            tabindex: 0,

            onfocusin: move |e| {
                info!("focused on editor: {:?}", e);
            },

            onfocusout: move |e| {
                info!("unfocused on editor: {:?}", e);
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
            EditorText {tabs, text, caret_col: caret_col(), caret_line: caret_line()},
            BottomStatusBar {tabs, caret_col: caret_col(), caret_line: caret_line()}
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

    let scroll_offset = use_resource(move || async move {
        if let Some(ref elem) = *element.read() {
            elem.get_scroll_offset().await.unwrap()
        } else {
            Vector2D::new(0.0, 0.0)
        }
    });

    let scroll_size = use_resource(move || async move {
        if let Some(ref elem) = *element.read() {
            elem.get_scroll_size().await.unwrap()
        } else {
            Size2D::new(0.0, 0.0)
        }
    });

    let scroll_offset= use_signal(move || scroll_offset.read_unchecked().clone());
    let scroll_size = use_signal(move || scroll_size.read_unchecked().clone());

    let lines = text.content.clone();

    rsx! {
        div {
            onmounted: move |e| {
                info!("mounted line: {:?}", e);
                element.set(Some(e.data()));
            },

            style: "background-color: purple; flex: 1; overflow-y: auto; flex: 1;",
            for (i, line) in lines.into_iter().enumerate() {

                EditorLine {
                    content: line, 
                    line: i, 
                    caret_col: caret_col, 
                    caret_line: caret_line, 
                    scroll_offset: scroll_offset,
                    scroll_size: scroll_size,
                }
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
    scroll_offset: Signal<Option<Vector2D<f64, Pixels>>>,
    scroll_size: Signal<Option<Size2D<f64, Pixels>>>,
) -> Element {
    
    let mut element: Signal<Option<Rc<MountedData>>> = use_signal(|| None);

    let _ = use_resource(move || async move {

        if line == caret_line() {
            if let Some(ref elem) = *element.read() {
                let client_rect = elem.get_client_rect().await.unwrap();
                let Some(scroll_offset) = scroll_offset.read().clone() else {
                    return;
                };
                let Some(scroll_size) = scroll_size.read().clone() else {
                    return;
                };

                dbg!(client_rect, scroll_offset, scroll_size);
    
                let is_visible = client_rect.min_y() >= scroll_offset.y &&
                                 client_rect.min_y() <= (scroll_offset.y + scroll_size.height);
    
                if !is_visible {
                    let _ = elem.scroll_to(ScrollBehavior::Instant).await;
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
                        (true, true) => "font-family: monospace; background-color: yellow; font-size: 16px;",
                        _ => "font-family: monospace; font-size: 16px;"
                    },
                    "{c}"
                }
            }

            if content.len() == caret_col() && line == caret_line(){
                span {
                    style: "background-color: yellow;",

                    "-"
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
