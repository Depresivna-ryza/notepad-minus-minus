use dioxus::prelude::*;
use tracing::info;
use std::path::PathBuf;
use crate::models::file_system::{FileSystemItem, FileSystem};
use crate::views::file_explorer::context_menu::{RightClickMenuHandler, RightClickMenu};
use crate::models::tabs::Tabs;
use dioxus_heroicons::{mini::Shape, Icon};

#[component]
pub fn File(file: PathBuf) -> Element {
    let mut right_click_menu_handler = use_context_provider(|| RightClickMenuHandler::new());
    let mut file_system = use_context::<Signal<FileSystem>>();

    let file_name = file.file_name().unwrap().to_str().unwrap();

    let open = {
        let file = file.clone();

        move |_| {
            let mut tabs = use_context::<Signal<Tabs>>();
            tabs.write().open_tab(file.clone());
        }
    };

    let focus = {
        let file = file.clone();

        move |_| {
            file_system.write().change_focus(&file);
        }
    };

    let open_right_click_menu = {
        let file = file.clone();

        move |event: MouseEvent| {
            right_click_menu_handler.handle_right_click(event);
            file_system.write().change_focus(&file);
        }
    };

    rsx!(
        div {
            class: if file_system.read().is_focused(&file) {
                "item-text-selected"
            } else {
                "file-text"
            },
            
            ondoubleclick: open,
            onclick: focus,
            oncontextmenu: open_right_click_menu,

            Icon {
                icon: Shape::Document,
            }
            
            span {
                " {file_name}"
            }
        }
        
        if right_click_menu_handler.is_open() {
            RightClickMenu { fs_item: FileSystemItem::File(file.clone()) }
        }
    )
}
