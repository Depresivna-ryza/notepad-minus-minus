use dioxus::prelude::*;
use tracing::info;
use std::path::PathBuf;
use crate::views::file_explorer::context_menu::{RightClickMenuState, RightClickMenu};
use crate::models::{
    files::{FileSystem, DirectoryItem},
    tabs::Tabs,
};


#[component]
pub fn File(file: PathBuf) -> Element {
    let file_name = file.file_name().unwrap().to_str().unwrap();
    
    let mut right_click_menu_state = use_context_provider(|| RightClickMenuState::new());

    let mut state = use_context::<Signal<FileSystem>>();

    let file1 = file.clone();
    let file2 = file.clone();
    let file3 = file.clone();

    rsx!(
        div {
            class: if state.read().is_focused(&file) {
                "item-text-selected"
            } else {
                "item-text"
            },
            
            ondoubleclick: move |_| {
                info!("File clicked: {:?}", file.clone());

                let mut tabs = use_context::<Signal<Tabs>>();
                tabs.write().open_tab(file1.clone());

            },

            onclick: move |_| {
                state.write().change_focus(&file2);
            },
            
            oncontextmenu: move |event: MouseEvent| {
                right_click_menu_state.handle_right_click(event);
                state.write().change_focus(&file3);
            },
            
            " {file_name}"
        }
        
        if right_click_menu_state.is_open() {
            RightClickMenu { directory_item: DirectoryItem::File(file.clone()) }
        }
    )
}
