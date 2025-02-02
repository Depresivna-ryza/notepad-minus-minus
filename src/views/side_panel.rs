use dioxus::prelude::*;
use dioxus_heroicons::{mini::Shape, IconButton};

use crate::models::panels::ShownPanels;

#[component]
pub fn SidePanelIcon(title: String, icon: Shape, on_click: Callback<(), ()>) -> Element {
    rsx! {
        IconButton {
            icon: icon,
            size: 35,
            title: title,
            onclick: move |_| on_click(()),
        }
    }
}

#[component]
pub fn SidePanel(shown_panels: ShownPanels) -> Element {

    rsx! {
        div {
            style: "display: flex; flex-direction: column; width: 50px; background-color: #eee;",
            SidePanelIcon { 
                title: "FileTree".to_string(),
                icon: Shape::Folder,
                on_click: move || { 
                    println!("FileTree clicked"); 
                    let val = *shown_panels.file_tree.read();
                    shown_panels.file_tree.set(!val);
                    dbg!(shown_panels.file_tree.peek());
                }
            },
            SidePanelIcon { 
                title: "Sessions".to_string(), 
                icon: Shape::User,
                on_click: move || { 
                    println!("Sessions clicked"); 
                    let val = *shown_panels.sessions.read();
                    shown_panels.sessions.set(!val);
                    dbg!(shown_panels.sessions.peek());
                }
            },
            SidePanelIcon { 
                title: "Search".to_string(),
                icon: Shape::MagnifyingGlass,
                on_click: move || { 
                    println!("Search clicked");
                    let val = *shown_panels.search.read();
                    shown_panels.search.set(!val);
                    dbg!(shown_panels.search.peek());
                }
            },
            SidePanelIcon { 
                title: "History".to_string(), 
                icon: Shape::Clock,
                on_click: move || { 
                    println!("History clicked"); 
                    let val = *shown_panels.history.read();
                    shown_panels.history.set(!val);
                    dbg!(shown_panels.history.peek());
                }
            },
            SidePanelIcon { 
                title: "Terminal".to_string(), 
                icon: Shape::CommandLine,
                on_click: move || { 
                    println!("Terminal clicked"); 
                    let val = *shown_panels.terminal.read();
                    shown_panels.terminal.set(!val);
                    println!("Terminal value: {}", shown_panels.terminal.peek());
                    dbg!(shown_panels.terminal.peek());
                }
            },
        }
    }
}
