use std::path::PathBuf;

use dioxus::prelude::*;
use tracing::info;

#[component]
pub fn EditorTabs(
    opened_tabs: Signal<Vec<PathBuf>>,
    current_file: Signal<Option<PathBuf>>,
) -> Element {
    rsx! {
        div {
            style: "background-color: pink; height: 70px; display: flex; overflow-x: auto;",

            for tab in opened_tabs.iter() {
                Tab { file_path: tab.clone(), current_file }
            }
        }
    }
}

#[component]
pub fn Tab(file_path: ReadOnlySignal<PathBuf>, current_file: Signal<Option<PathBuf>>) -> Element {
    let file_name_short = use_memo(move || match file_path().file_name() {
        None => "Invalid file".to_string(),
        Some(f) => match f.to_str() {
            None => "Invalid file".to_string(),
            Some(s) => s.chars().take(10).collect::<String>(),
        },
    });

    let is_current = use_memo(move || current_file() == Some(file_path()));

    rsx! {
        div {
            style: if is_current() {
                "min-width: 300px; border: 3px solid blue; margin: 1px; padding: 5px; background-color: yellow; color: black;"
            } else {
                "min-width: 300px; border: 3px solid blue; margin: 1px; padding: 5px;"
            },
            onclick: move |_| {
                current_file.replace(Some(file_path()));
                info!("current file changed to: {:?}", file_path);
            },
            "{file_name_short}"
        }
    }
}