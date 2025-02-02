use std::path::PathBuf;

use dioxus::{html::g::strikethrough_thickness, prelude::*};
use tracing::info;

use crate::models::tabs::{Tabs, Tab};

#[component]
pub fn EditorTabs(tabs: Signal<Tabs>) -> Element {
    rsx! {
        div {
            style: "background-color: pink; height: 70px; display: flex; overflow-x: auto;",

            for tab in tabs.read().opened_tabs.iter() {
                TabView { file: tab.clone(), tabs }
            }
        }
    }
}

#[component]
pub fn TabView(file: ReadOnlySignal<Tab>, tabs: Signal<Tabs>) -> Element {
    let file_name_short = use_memo(move || match file().file.path.file_name() {
        None => "Invalid file".to_string(),
        Some(f) => match f.to_str() {
            None => "Invalid file".to_string(),
            Some(s) => s.chars().take(10).collect::<String>(),
        },
    });

    let is_current = use_memo(move || tabs.read().current_file == Some(file().file.path));
    let exists = use_memo(move || file().file.path.exists());

    use_future(move || async move {
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;

            let path = file().file.path.clone();
            let exists = path.exists();
            
            tabs.write().update_existance(path, exists);
            
        }
    });

    rsx! {
        div {
            style: "min-width: 300px; border: 3px solid blue; margin: 1px; padding: 5px;".to_string() + 

            match is_current() {
                true => "background-color: yellow; color: black;",
                false => ""
            } +

            match exists() {
                true => "",
                false => "color: red; font-weight: bold; font-style: italic; text-decoration: line-through; text-decoration-style: wavy",
            },

            onclick: move |_| {
                tabs.write().set_current_file(file().file.path);
                info!("current file changed to: {:?}", file().file.path);
            },

            a {
                "{file_name_short}"
            }

            button {
                style: "margin-left: 5px;",
                onclick: move |_| {
                    tabs.write().close_tab(file().file.path);
                    info!("tab closed: {:?}", file().file.path);
                },
                "X"
            }
        }
    }
}
