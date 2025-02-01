use dioxus::prelude::*;
use crate::models::files::{Dir, DirectoryItem, DirectoryItems, FileSystem};
use crate::views::file_explorer::{
    file::File,
    dialogs::{RightClickMenu, RightClickMenuState},
};

#[component]
pub fn Directory(dir: Dir) -> Element {
    let dir_name = dir.path.file_name().unwrap().to_str().unwrap();
    
    let mut right_click_menu_state = use_context_provider(|| RightClickMenuState::new());
    
    let mut state = use_context::<Signal<FileSystem>>();

    let opened_string = match dir.children {
        DirectoryItems::OpenedDirectory(_) => "[v]",
        DirectoryItems::ClosedDirectory => "[>]",
    };

    let path1 = dir.path.clone();
    let path2 = dir.path.clone();

    rsx!(
        div {
            class: "item-text",
            div {
                a {
                    style: "white-space: nowrap;",
                    onclick: move |_| {
                        state.write().find(&path1);
                    },
                    " {opened_string} "
                }
                
                a {
                    class: if state.read().is_focused(&dir.path) {
                        "item-text-selected"
                    } else {
                        "item-text"
                    },
                    
                    onclick: move |_| {
                        state.write().change_focus(&path2);
                    },
                    
                    oncontextmenu: move |event: MouseEvent| {
                        right_click_menu_state.handle_right_click(event);
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
