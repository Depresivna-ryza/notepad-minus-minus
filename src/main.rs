pub mod models;
pub mod views;

use dotenvy::dotenv;
use models::panels::ShownPanels;
use models::tabs::Tabs;
use views::editor::Editor;
use views::fileexplorer::FileExplorer;
use views::sessionexplorer::SessionsExplorer;
use views::side_panel::SidePanel;
use views::tabs::EditorTabs;

use dioxus::prelude::*;
use views::terminal::Terminal;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dotenv().ok();
    launch(Layout);
}

#[component]
pub fn Layout() -> Element {
    let tabs = use_signal(Tabs::new);
    let shown_panels = ShownPanels::new();

    rsx! {

        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        
        div {
            style: "display: flex; flex-direction: row; width: 100vw ; height: 100vh;",
            SidePanel {shown_panels}
            div {  
                style: "display: flex; flex-direction: column; flex: 1; max-height: 100%;",
                div {
                    style: "display: flex; flex-direction: row; flex: 1; max-height: 100%; overflow: hidden;",
                    LeftPanel {tabs}
                    RightPanel {tabs}
                }
                Terminal {
                    hidden: shown_panels.terminal
                }
            }
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
            style: "display: flex; flex-direction: column; background-color: #ddd; flex: 1;overflow: hidden;",
            EditorTabs {tabs}
            Editor {tabs}
        }
    }
}
