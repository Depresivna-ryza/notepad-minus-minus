pub mod views;
pub mod models;

use std::path::PathBuf;

use views::fileexplorer::FileExplorer;
use views::sessionexplorer::SessionsExplorer;
use views::editor::Editor;
use views::tabs::EditorTabs;


use dioxus::prelude::*;


const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(Layout);
}

#[component]
pub fn Layout() -> Element {
    let opened_tabs: Signal<Vec<PathBuf>> = use_signal(Vec::new);
    let current_file: Signal<Option<PathBuf>> = use_signal(|| None);

    rsx! {

        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        div {
            style: "display: flex; flex-direction: row; width: 100vw ; height: 100vh;",
            LeftPanel {opened_tabs, current_file}
            RightPanel {opened_tabs, current_file}
        }

    }
}

#[component]
pub fn LeftPanel(
    opened_tabs: Signal<Vec<PathBuf>>,
    current_file: Signal<Option<PathBuf>>,
) -> Element {
    rsx! {
        div {
            style: "display: flex; flex-direction: column; width: 20%; background-color: #eee;",
            FileExplorer {opened_tabs: opened_tabs, current_file}
            SessionsExplorer {}
        }
    }
}

#[component]
pub fn RightPanel(
    opened_tabs: Signal<Vec<PathBuf>>,
    current_file: Signal<Option<PathBuf>>,
) -> Element {
    rsx! {
        div {
            style: "display: flex; flex-direction: column; width: 80%; background-color: #ddd;",
            EditorTabs {opened_tabs, current_file}
            Editor {current_file}
        }
    }
}


