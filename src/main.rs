use std::{fs::read_to_string, path::PathBuf};

use dioxus::prelude::*;
use rfd::FileDialog;
use tracing::info;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

static OPENED_TABS: GlobalSignal<Vec<PathBuf>> = Signal::global(Vec::new);
static CURRENT_FILE: GlobalSignal<Option<PathBuf>> = Signal::global(|| None);

fn main() {
    dioxus::launch(Layout);
}

#[component]
pub fn Layout() -> Element {
    rsx! {

        document::Link { rel: "icon", href: FAVICON }

        document::Link { rel: "stylesheet", href: MAIN_CSS }

        div {
            style: "display: flex; flex-direction: row; width: 100vw ; height: 100vh;",
            LeftPanel {}
            RightPanel {}
        }

    }
}

#[component]
pub fn LeftPanel() -> Element {
    rsx! {
        div {
            style: "display: flex; flex-direction: column; width: 20%; background-color: #eee;",
            FileExplorer {}
            SessionsExplorer {}
        }
    }
}

#[component]
pub fn RightPanel() -> Element {
    rsx! {
        div {
            style: "display: flex; flex-direction: column; width: 80%; background-color: #ddd;",
            EditorTabs {}
            Editor {}
        }
    }
}

#[component]
pub fn FileExplorer() -> Element {
    let open_file = move |_| {
        if let Some(file_path) = FileDialog::new().pick_file() {
            OPENED_TABS.write().push(file_path.clone());
            info!("File opened: {:?}", file_path);

            CURRENT_FILE.write().replace(file_path.clone());
            info!("current file changed to: {:?}", file_path);
        }
    };

    rsx! {
        div {
            style: "flex: 1; background-color: lightgreen;",
            "FileExplorer"
            button {
                onclick: open_file,
                "Open File"
            }
        }
    }
}

#[component]
pub fn SessionsExplorer() -> Element {
    rsx! {
        div {
            style: "flex: 1; background-color: lightblue;",
            "SessionsExplorer content"
        }
    }
}

#[component]
pub fn EditorTabs() -> Element {
    let tabs = OPENED_TABS.read();

    rsx! {
        div {
            style: "background-color: pink; height: 70px; display: flex; overflow-x: auto;",

            for tab in tabs.iter() {
                Tab { file_path: tab.clone() }
            }
        }
    }
}

#[component]
pub fn Editor() -> Element {
    let current_file = CURRENT_FILE.read().clone();
    let file_content = match current_file {
        Some(file_path) => read_to_string(file_path).unwrap_or("NO FILE".to_string()),
        None => "NO FILE".to_string(),
    };

    rsx! {
        div {
            style: "background-color: red; flex: 1;",
            "{file_content}"
        }
    }
}

#[component]
pub fn Tab(file_path: ReadOnlySignal<PathBuf>) -> Element {
    let file_name_short = use_memo(move || {
        match file_path().file_name() {
            None => "Invalid file".to_string(),
            Some(f) => {
                match f.to_str() {
                    None => "Invalid file".to_string(),
                    Some(s) => s.chars().take(10).collect::<String>(),
                }
            }
        }
    });

    let is_current = use_memo(move || CURRENT_FILE() == Some(file_path()));

    rsx! {
        div {
            style: if is_current() {
                "min-width: 300px; border: 3px solid blue; margin: 1px; padding: 5px; background-color: yellow; color: black;"
            } else {
                "min-width: 300px; border: 3px solid blue; margin: 1px; padding: 5px;"
            },
            onclick: move |_| {
                CURRENT_FILE.write().replace(file_path());
                info!("current file changed to: {:?}", file_path);
            },
            "{file_name_short}"
        }
    }
}
