use dioxus::prelude::*;
use dioxus_heroicons::{mini::Shape, Icon};

use crate::models::tabs::{Tab, Tabs};

#[component]
pub fn EditorTabs(tabs: Signal<Tabs>) -> Element {
    rsx! {
        div {
            style: "background-color: rgb(36, 24, 32); height: 40px; display: flex; overflow-x: auto;",

            for tab in tabs.read().opened_tabs.iter() {
                TabView { file: tab.clone(), tabs }
                div {
                    style: "width: 1px; background-color: rgba(73, 173, 255, 0.15); height: 100%;",
                }
            }
        }
    }
}

#[component]
pub fn TabView(file: ReadOnlySignal<Tab>, tabs: Signal<Tabs>) -> Element {
    let file_name_short = use_memo(move || match file().file.path.file_name() {
        None => "Invalid file".to_string(),
        Some(f) => match f.to_str() {
            None => "Invalid file".to_string(),
            Some(s) => s.chars().collect::<String>(),
        },
    });

    let is_current = use_memo(move || tabs.read().current_file == Some(file().file.path));
    let exists = use_memo(move || file().file.path.exists());

    use_future(move || async move {
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;

            let path = file().file.path.clone();
            let exists = path.exists();

            tabs.write().update_existance(path, exists);
        }
    });

    rsx! {
        div {
            class: "tab".to_string() +
            match is_current() {
                true => " highlighted",
                false => " "
            } +

            match exists() {
                true => "",
                false => " non-exists",
            },
            onclick: move |_| {
                tabs.write().set_current_file(file().file.path);
                // info!("current file changed to: {:?}", file().file.path);
            },
            div {
                style: "width: 100%; height: 100%; display: flex; overflow: hidden; align-items: center; white-space: nowrap;",
                "{file_name_short}"
            }

            div {
                style: "margin-left: 5px; width: 20px; height: 20px; display: flex; justify-content: center; align-items: center;",
                onclick: move |_| {
                    tabs.write().close_tab(file().file.path);
                    // info!("tab closed: {:?}", file().file.path);
                },
                Icon {
                    size: 20,
                    icon: Shape::XMark
                }
            }
        }
    }
}
