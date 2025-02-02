use dioxus::prelude::*;
use crate::models::files::{Dir, DirectoryItem, DirectoryItems, FileSystem};
use crate::views::file_explorer::{
    file::File,
    context_menu::{RightClickMenu, RightClickMenuHandler},
};

#[component]
pub fn Directory(dir: Dir) -> Element {
    let mut right_click_menu_handler = use_context_provider(|| RightClickMenuHandler::new());
    let mut focus_state = use_context::<Signal<FileSystem>>();

    let dir_name = dir.path.file_name().unwrap().to_str().unwrap();

    let opened_string = match dir.children {
        DirectoryItems::OpenedDirectory(_) => "[v]",
        DirectoryItems::ClosedDirectory => "[>]",
    };

    let item_text_class = if focus_state.read().is_focused(&dir.path) {
        "item-text-selected"
    } else {
        "item-text"
    };

    let open_close = {
        let path = dir.path.clone();

        move |_| {
            focus_state.write().find(&path);
        }
    };

    let change_focus = {
        let path = dir.path.clone();

        move |_| {
            focus_state.write().change_focus(&path);
        }
    };

    let open_right_click_menu = {
        let path = dir.path.clone();

        move |event: MouseEvent| {
            right_click_menu_handler.handle_right_click(event);
            focus_state.write().change_focus(&path);
        }
    };

    rsx!(
        div {
            class: "item-text",
            div {
                a {
                    style: "white-space: nowrap;",
                    onclick: open_close,

                    " {opened_string} "
                }
                
                a {
                    class: item_text_class,
                    onclick: change_focus,
                    oncontextmenu: open_right_click_menu,

                    " {dir_name} "
                }
            }
            
            if right_click_menu_handler.is_open() {
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
