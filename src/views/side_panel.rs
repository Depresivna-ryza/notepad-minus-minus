use dioxus::prelude::*;
use dioxus_heroicons::{mini::Shape, Icon};

use crate::models::panels::ShownPanels;

#[component]
pub fn SidePanelIcon(
    title: String,
    icon: Shape,
    on_click: Callback<(), ()>,
    selected: bool,
) -> Element {
    rsx! {
        button {
            class: "side-panel-icon".to_string() + if selected { " selected" } else { "" },
            fill: "red",
            title: title,
            onclick: move |_| on_click(()),
            Icon {
                fill: "rgb(183, 188, 255)",
                size: 30,
                icon: icon,
            }
        }
        div {
            style: "height: 1px; background-color: rgb(26, 28, 48); width: 100%; align-self: center;",
        }
    }
}

#[component]
pub fn SidePanel(shown_panels: ShownPanels) -> Element {
    rsx! {
        div {
            style: "display: flex; flex-direction: column; width: 50px; background-color: rgb(15, 16, 24);",
            SidePanelIcon {
                title: "FileTree".to_string(),
                icon: Shape::Folder,
                on_click: move || {
                    println!("FileTree clicked");
                    let val = *shown_panels.file_tree.read();
                    shown_panels.file_tree.set(!val);
                    dbg!(shown_panels.file_tree.peek());
                },
                selected: *shown_panels.file_tree.read()
            },
            SidePanelIcon {
                title: "Sessions".to_string(),
                icon: Shape::User,
                on_click: move || {
                    println!("Sessions clicked");
                    let val = *shown_panels.sessions.read();
                    shown_panels.sessions.set(!val);
                    dbg!(shown_panels.sessions.peek());
                },
                selected: *shown_panels.sessions.read()
            },
            SidePanelIcon {
                title: "Search".to_string(),
                icon: Shape::MagnifyingGlass,
                on_click: move || {
                    println!("Search clicked");
                    let val = *shown_panels.search.read();
                    shown_panels.search.set(!val);
                    dbg!(shown_panels.search.peek());
                },
                selected: *shown_panels.search.read()
            },
            SidePanelIcon {
                title: "History".to_string(),
                icon: Shape::Clock,
                on_click: move || {
                    println!("History clicked");
                    let val = *shown_panels.history.read();
                    shown_panels.history.set(!val);
                    dbg!(shown_panels.history.peek());
                },
                selected: *shown_panels.history.read()
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
                },
                selected: *shown_panels.terminal.read()
            },
        }
    }
}
