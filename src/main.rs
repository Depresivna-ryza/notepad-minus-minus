pub mod models;
pub mod views;

use models::network::P2PNetwork;
use models::sessions::Sessions;
use models::tabs::Tabs;
use views::editor::Editor;
use views::fileexplorer::FileExplorer;
use views::sessionexplorer::SessionsExplorer;
use views::tabs::EditorTabs;

use dioxus::prelude::*;
use std::sync::{Arc, RwLock};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(Layout);
}

#[component]
pub fn Layout() -> Element {
    let tabs = use_signal(Tabs::new);
    let sessions: Signal<Sessions> = use_signal(Sessions::new);
    // let received_messages = Arc::new(RwLock::new(Vec::new()));
    // let received_messages_clone = Arc::clone(&received_messages);

    // Initialize the P2P network
    // use_coroutine(|rx: UnboundedReceiver<()>| {
    //     to_owned![received_messages_clone];
    //     async move {
    //         match P2PNetwork::new().await {
    //             Ok((mut network, mut message_receiver)) => {
    //                 // Spawn a task to handle incoming messages
    //                 tokio::spawn(async move {
    //                     while let Some(msg) = message_receiver.recv().await {
    //                         received_messages_clone.write().unwrap().push(msg);
    //                     }
    //                 });

    //                 // Run the network
    //                 if let Err(e) = network.run().await {
    //                     eprintln!("Network error: {}", e);
    //                 }
    //             }
    //             Err(e) => eprintln!("Failed to initialize P2P network: {}", e),
    //         }
    //     }
    // });

    rsx! {

        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        div {
            style: "display: flex; flex-direction: row; width: 100vw ; height: 100vh;",
            LeftPanel {tabs, sessions}
            RightPanel {tabs}
        }

    }
}

#[component]
pub fn LeftPanel(tabs: Signal<Tabs>, sessions: Signal<Sessions>) -> Element {
    rsx! {
        div {
            style: "display: flex; flex-direction: column; width: 20%; background-color: #eee;",
            FileExplorer {tabs}
            SessionsExplorer {sessions}
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
