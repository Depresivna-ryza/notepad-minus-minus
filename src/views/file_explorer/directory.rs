use dioxus::prelude::*;
use tracing::info;
use crate::models::files::{Dir, DirectoryItem, DirectoryItems, FileSystem};
use crate::views::file_explorer::{
    file::File,
    context_menu::{RightClickMenu, RightClickMenuState},
};

#[component]
pub fn Directory(dir: Dir) -> Element {
    let dir_name = dir.path.file_name().unwrap().to_str().unwrap();
    
    let mut right_click_menu_state = use_context_provider(|| RightClickMenuState::new());
    
    let mut focus_state = use_context::<Signal<FileSystem>>();

    let opened_string = match dir.children {
        DirectoryItems::OpenedDirectory(_) => "[v]",
        DirectoryItems::ClosedDirectory => "[>]",
    };

    let path1 = dir.path.clone();
    let path2 = dir.path.clone();
    let path3 = dir.path.clone();

    rsx!(
        div {
            class: "item-text",
            div {
                a {
                    style: "white-space: nowrap;",
                    onclick: move |_| {
                        focus_state.write().find(&path1);
                    },
                    " {opened_string} "
                }
                
                a {
                    class: if focus_state.read().is_focused(&dir.path) {
                        "item-text-selected"
                    } else {
                        "item-text"
                    },
                    
                    onclick: move |_| {
                        focus_state.write().change_focus(&path2);
                    },

                    tabindex: 0,
                    
                    onfocusout: move |_| {
                        right_click_menu_state.close_menu();
                        focus_state.write().clear_focus();
                    },

                    oncontextmenu: move |event: MouseEvent| {
                        right_click_menu_state.handle_right_click(event);
                        focus_state.write().change_focus(&path3);
                    },
                    " {dir_name} "
                }
            }
            
            if right_click_menu_state.is_open() {
                RightClickMenu { directory_item: DirectoryItem::Directory(dir.clone()) }
            }
            
            if let DirectoryItems::OpenedDirectory(dir_items) = dir.children {
                for item in dir_items.iter() {
                    if let DirectoryItem::Directory(dir) = item {
                        Directory { dir: dir.clone()}
                    }
                }

                for item in dir_items.iter() {
                    if let DirectoryItem::File(file) = item {
                        File { file: file.clone() }
                    }
                }
            }
        }
    )
}
