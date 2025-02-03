pub mod models;
pub mod views;

use futures::FutureExt;
use models::tabs::Tabs;
use views::editor::Editor;
use views::fileexplorer::FileExplorer;
use views::sessionexplorer::SessionsExplorer;
use views::tabs::EditorTabs;

use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(Layout);
}

#[component]
pub fn Layout() -> Element {
    let tabs = use_signal(Tabs::new);
    //let sessions: Signal<Sessions> = use_signal(Sessions::new);

    rsx! {

        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        div {
            style: "display: flex; flex-direction: row; width: 100vw ; height: 100vh;",
            LeftPanel {tabs}
            RightPanel {tabs}
        }

    }
}

#[component]
pub fn LeftPanel(tabs: Signal<Tabs>) -> Element {
    rsx! {
        div {
            style: "display: flex; flex-direction: column; width: 20%; background-color: #eee;",
            FileExplorer {tabs}
            SessionsExplorer {}
        }
    }
}

#[component]
pub fn RightPanel(tabs: Signal<Tabs>) -> Element {
    rsx! {
        div {
            style: "display: flex; flex-direction: column; width: 80%; background-color: #ddd;",
            EditorTabs {tabs}
            Editor {tabs}
        }
    }
}
