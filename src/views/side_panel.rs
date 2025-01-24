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
                on_click: || { println!("FileTree clicked"); }
            },
            SidePanelIcon { 
                title: "Search".to_string(),
                icon: Shape::MagnifyingGlass,
                on_click: || { println!("Search clicked"); }
            },
            SidePanelIcon { 
                title: "Terminal".to_string(), 
                icon: Shape::CommandLine,
                on_click: move || { 
                    println!("Terminal clicked"); 
                    let val = shown_panels.terminal.read().clone();
                    shown_panels.terminal.set(!val);
                    println!("Terminal value: {}", shown_panels.terminal.peek());
                }
            },
        }
    }
}
