use std::fs::read_to_string;
use crate::models::{tabs::Tabs, text::Text};

use dioxus::prelude::*;

#[component]
pub fn Editor(tabs: Signal<Tabs>) -> Element {
    let text: Signal<Option<Text>> = use_signal( || 
        match tabs.read().current_file {
            Some(ref file_path) => read_to_string(file_path).ok().map(Text::new),
            None => None,
        }
    );

    // let text = use_signal(|| file_content);

    let caret_col = use_memo(move || 
        match text.read().clone() {
            Some(text) => text.caret_column,
            None => 0,
        }
    );

    let caret_line: Memo<usize> = use_memo(move || 
        match text.read().clone() {
            Some(text) => text.caret_line,
            None => 0,
        }
    );

    rsx!{
        div {
            style: "display: flex; flex-direction: column; flex: 1; justify-content: space-between; height: 10px;",
            TopStatusBar {tabs, text},
            EditorText {tabs, text},
            BottomStatusBar {tabs, caret_col: caret_col(), caret_line: caret_line()} 
        }
    }
}


#[component]
pub fn EditorText(tabs: Signal<Tabs>, text: Signal<Option<Text>>) -> Element {
    let current_file = use_memo(move || tabs.read().current_file.clone());

    let text: Option<Text> = match current_file() {
        Some(file_path) => read_to_string(file_path).ok().map(Text::new),
        None => None,
    };


    // let text: Signal<Option<Text>> = use_signal( || 
    //     match tabs.read().current_file {
    //         Some(ref file_path) => read_to_string(file_path).ok().map(Text::new),
    //         None => None,
    //     }
    // );


    // let Some(ref text) = *text.read() else {
    let Some(ref text) = text else {
        return rsx!{
            div {
                style: "background-color: purple; flex: 1; color: red;",
                "<NO FILE>"
            }
        }
    };

    rsx!{
        div {
            style: "background-color: purple; flex: 1; overflow-y: auto; flex: 1;",
            ul {
                style: "padding: 0;",
                for text_line in text.lines() {
                    li {
                        style: "list-style-type: none;",
                        "{text_line}"
                    }
                }
            }
        }
    }
}

#[component]
pub fn TopStatusBar(tabs: ReadOnlySignal<Tabs>, text: Signal<Option<Text>>) -> Element {
    rsx!{
        div {
            style: "background-color: blue; height: 40px; display: flex; justify-content: space-between; align-items: center; ",
            Breadcrumbs {tabs},
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
    caret_line: ReadOnlySignal<usize>) -> Element {
    rsx!{
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
pub fn Breadcrumbs(tabs: ReadOnlySignal<Tabs>) -> Element {
    let path: Option<Vec<String>> = tabs().current_file.map(|p| {
        p.iter().map(|p| p.to_string_lossy().to_string()).collect()
    });

    let Some(path) = path else {
        return rsx!{
            div {
                style: "background-color: green; height: 100%; display: flex; flex: 1; overflow-x: auto; white-space: nowrap;",
                "<No file selected>"
            }
        }
    };


    rsx!{
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


