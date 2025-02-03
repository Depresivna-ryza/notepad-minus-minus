use dioxus::prelude::*;
use tracing::info;
use std::path::PathBuf;
use crate::models::file_system::{FileSystemItem, FileSystem};
use crate::views::file_explorer::context_menu::{RightClickMenuHandler, RightClickMenu};
use crate::models::tabs::Tabs;


#[component]
pub fn File(file: PathBuf) -> Element {
    let mut right_click_menu_handler = use_context_provider(|| RightClickMenuHandler::new());
    let mut state = use_context::<Signal<FileSystem>>();

    let file_name = file.file_name().unwrap().to_str().unwrap();

    let file1 = file.clone();
    let file2 = file.clone();
    let file3 = file.clone();

    rsx!(
        div {
            class: if state.read().is_focused(&file) {
                "item-text selected"
            } else {
                "item-text"
            },
            style: "backgorund-color: red;",
            
            ondoubleclick: move |_| {
                // info!("File clicked: {:?}", file.clone());

                let mut tabs = use_context::<Signal<Tabs>>();
                tabs.write().open_tab(file1.clone());

            },

            onclick: move |_| {
                state.write().change_focus(&file2);
            },
            
            oncontextmenu: move |event: MouseEvent| {
                right_click_menu_handler.handle_right_click(event);
                state.write().change_focus(&file3);
            },
            
            "                {file_name}"
        }
        
        if right_click_menu_handler.is_open() {
            RightClickMenu { fs_item: FileSystemItem::File(file.clone()) }
        }
    )
}
