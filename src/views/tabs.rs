use std::path::PathBuf;

use dioxus::prelude::*;
use tracing::info;

use crate::models::tabs::Tabs;

#[component]
pub fn EditorTabs(
    tabs: Signal<Tabs>
) -> Element {
    rsx! {
        div {
            style: "background-color: pink; height: 70px; display: flex; overflow-x: auto;",

            for tab in tabs.read().opened_tabs.iter() {
                Tab { file_path: tab.clone(), tabs }
            }
        }
    }
}

#[component]
pub fn Tab(file_path: ReadOnlySignal<PathBuf>, tabs: Signal<Tabs>) -> Element {
    let file_name_short = use_memo(move || match file_path().file_name() {
        None => "Invalid file".to_string(),
        Some(f) => match f.to_str() {
            None => "Invalid file".to_string(),
            Some(s) => s.chars().take(10).collect::<String>(),
        },
    });

    let is_current = use_memo(move || tabs.read().current_file == Some(file_path()));

    rsx! {
        div {
            style: if is_current() {
                //the leading whitespace is necessary, otherwise the style breaks the app :)
                " min-width: 300px; border: 3px solid blue; margin: 1px; padding: 5px; background-color: yellow; color: black;"
            } else {
                " min-width: 300px; border: 3px solid blue; margin: 1px; padding: 5px;"
            },

            a {
                onclick: move |_| {
                    tabs.write().set_current_file(file_path());
                    info!("current file changed to: {:?}", file_path);
                },
                "{file_name_short}"
            }

            button {
                style: "margin-left: 5px;",
                onclick: move |_| {
                    tabs.write().close_tab(file_path());
                    info!("tab closed: {:?}", file_path);
                },
                "X"
            }
        }
    }
}