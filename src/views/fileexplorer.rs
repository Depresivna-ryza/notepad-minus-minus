use std::path::PathBuf;
use tracing::info;
use crate::models::{
    files::{Dir, DirectoryItem, DirectoryItems, FileSystem},
    tabs::Tabs,
};

use dioxus::prelude::*;
use rfd::{AsyncFileDialog, FileDialog};

#[component]
pub fn FileExplorer(tabs: Signal<Tabs>) -> Element {
    let mut file_system_state = use_context_provider(|| Signal::new(FileSystem::new()));
    use_context_provider(|| tabs);

    let change_root_directory = move |_| async move {
        if let Some(dir_path) = AsyncFileDialog::new().pick_folder().await {
            let mut root_dir = Dir::new(dir_path.path().to_path_buf());
            root_dir.open();
            file_system_state.replace(FileSystem::from(root_dir));
        }
    };

    rsx! {
        div {
            style: "flex: 1; background-color: lightgreen; overflow-x: auto; overflow-y: auto;",

            a {"FileExplorer"}

            button {
                onclick: change_root_directory,
                "Change root directory"
            }

            if let Some(dir) = file_system_state.read().root.clone() {
                Directory { dir }
            } else {
                div {
                    "No directory selected"
                }
            }
        }
    }
}

#[component]
pub fn Directory(dir: Dir) -> Element {
    let dir_name = dir.path.file_name().unwrap().to_str().unwrap();

    let opened_string = match dir.children {
        DirectoryItems::OpenedDirectory(_) => "[v]",
        DirectoryItems::ClosedDirectory => "[>]",
    };
    
    let mut is_menu_open = use_signal(|| false);
    let mut menu_position = use_signal(|| (0.0, 0.0));

    rsx!(
        div {
            style: "color: darkred; border: 1px solid darkred; margin: 5px 0px 5px 20px;",
            div {
                a {
                    style: "white-space: nowrap;",
                    onclick: move |_| {
                        info!("File clicked: {:?}", dir.path);

                        let mut state = use_context::<Signal<FileSystem>>();
                        state.write().find(&dir.path);
                    },
                    " {opened_string} "
                }
                
                a {
                    style: "white-space: nowrap;",
                    oncontextmenu: move |event: MouseEvent| {
                        event.prevent_default();
                        is_menu_open.set(true);
                        let coordinates = event.client_coordinates();
                        menu_position.set((coordinates.x, coordinates.y));
                    },
                    " {dir_name} "
                    if *is_menu_open.read() {
                        RightClickMenu { is_menu_open, menu_position }
                    }
                }
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

#[component]
pub fn RightClickMenu(mut is_menu_open: Signal<bool>, menu_position: Signal<(f64, f64)>) -> Element {
    let mut close_menu = move || {
        is_menu_open.set(false);
    };

    info!("Menu position: {:?}", menu_position.read());
    
    let menu_position = menu_position.read();
    
    rsx!(
        div {
            style: "
                position: absolute;
                top: {menu_position.1}px;
                left: {menu_position.0}px;
                background: white;
                border: 1px solid black;
                box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
                padding: 10px;
                z-index: 1000;
            ",
            onclick: move |_| close_menu(),

            // Menu options
            p { "Option 1" }
            p { "Option 2" }
            p { "Option 3" }
        }
    )
}

#[component]
pub fn File(file: PathBuf) -> Element {
    let file_name = file.file_name().unwrap().to_str().unwrap();

    rsx!(
        div {
            onclick: move |_| {
                info!("File clicked: {:?}", file.clone());

                let mut tabs = use_context::<Signal<Tabs>>();
                tabs.write().open_tab(file.clone());

            },
            style: "color: blue; border: 1px solid blue; margin: 5px 0px 5px 20px;",
            "{file_name}"
        }
    )
}
