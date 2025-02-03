use crate::models::file_system::{Directory, FileSystem, FileSystemItem};
use crate::views::file_explorer::{context_menu::RightClickMenuHandler, file::File};
use dioxus::prelude::*;
use std::path::PathBuf;

#[component]
pub fn DirectoryComponent(path: PathBuf) -> Element {
    let mut right_click_menu_handler = use_context::<Signal<RightClickMenuHandler>>();
    let mut file_system = use_context::<Signal<FileSystem>>();

    let opened_string = if file_system.read().directory_is_opened(&path) {
        "v"
    } else {
        ">"
    };

    let item_text_class = if file_system.read().is_focused(&path) {
        "item-text selected"
    } else {
        "item-text"
    };

    let open_close = {
        let mut file_system = file_system;
        let path = path.clone();

        move |_| {
            file_system.write().open_close_directory(&path);
        }
    };

    let change_focus = {
        let mut file_system = file_system;
        let path = path.clone();

        move |_| {
            file_system.write().change_focus(&path);
        }
    };

    let open_right_click_menu = {
        let path = path.clone();

        move |event: MouseEvent| {
            right_click_menu_handler
                .write()
                .set_fs_item(FileSystemItem::Directory(Directory::from(&path)));
            right_click_menu_handler.write().handle_right_click(event);
            file_system.write().change_focus(&path);
        }
    };

    rsx!(
        div {
            class: "item-text",
            div {
                style: "display: flex; flex-direction: row; align-items: center; margin-left: 10px; margin-top: 5px;",
                div {
                    style: "white-space: nowrap; cursor: pointer;",
                    onclick: open_close,
                    " {opened_string} "
                }

                div {
                    class: item_text_class,
                    onclick: change_focus,
                    oncontextmenu: open_right_click_menu,

                    " { file_system.read().get_directory_name(&path) } "
                }
            }

            div {
                style: " border-left: 1px solid rgb(131, 58, 58); margin-left: 10px; ",
                if file_system.read().directory_is_opened(&path) {
                    for item in file_system.read().get_directory_children(&path).iter() {
                        if let FileSystemItem::Directory(dir) = item {
                            DirectoryComponent { path: dir.get_path().clone() }

                        }
                    }

                    for item in file_system.read().get_directory_children(&path).iter() {
                    if let FileSystemItem::File(file) = item {
                        File { file: file.clone() }
                    }
                }
            }
            }
        }
    )
}
