use std::path::PathBuf;
use tracing::info;
use crate::models::{
    files::{Dir, DirectoryItem, DirectoryItems, FileSystem},
    tabs::Tabs,
};

use dioxus::prelude::*;
use rfd::{AsyncFileDialog, FileDialog};

#[derive(Clone, Copy)]
struct RightClickMenuState {
    is_open: Signal<bool>,
    position: Signal<(f64, f64)>,
}

impl RightClickMenuState {
    pub fn handle_right_click(&mut self, event: MouseEvent) {
        event.prevent_default();
        self.is_open.set(true);
        let coordinates = event.client_coordinates();
        self.position.set((coordinates.x, coordinates.y));
    }
    
    pub fn close_menu(&mut self) {
        self.is_open.set(false);
    }
}

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
    
    let right_click_menu_state = use_context_provider(|| RightClickMenuState {
        is_open: Signal::new(false),
        position: Signal::new((0.0, 0.0)),
    });

    let opened_string = match dir.children {
        DirectoryItems::OpenedDirectory(_) => "[v]",
        DirectoryItems::ClosedDirectory => "[>]",
    };

    let mut right_click_menu_state = use_context::<RightClickMenuState>();

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
                        right_click_menu_state.handle_right_click(event);
                    },
                    " {dir_name} "
                }
            }
            
            if *right_click_menu_state.is_open.read() {
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

#[component]
pub fn File(file: PathBuf) -> Element {
    let file_name = file.file_name().unwrap().to_str().unwrap();
    
    let right_click_menu_state = use_context_provider(|| RightClickMenuState {
        is_open: Signal::new(false),
        position: Signal::new((0.0, 0.0)),
    });
    
    let mut right_click_menu_state = use_context::<RightClickMenuState>();

    rsx!(
        div {
            style: "color: blue; border: 1px solid blue; margin: 5px 0px 5px 20px;",
            
            onclick: move |_| {
                info!("File clicked: {:?}", file.clone());

                let mut tabs = use_context::<Signal<Tabs>>();
                tabs.write().open_tab(file.clone());

            },
            
            oncontextmenu: move |event: MouseEvent| {
                right_click_menu_state.handle_right_click(event);
            },
            
            "{file_name}"
        }
        
        if *right_click_menu_state.is_open.read() {
            RightClickMenu { directory_item: DirectoryItem::File(file.clone()) }
        }
    )
}

#[component]
pub fn RightClickMenu(directory_item: DirectoryItem) -> Element {
    let mut right_click_menu_state = use_context::<RightClickMenuState>();

    info!("Menu position: {:?}", right_click_menu_state.position.read());
    
    info!("Directory item: {:?}", directory_item);

    let menu_position = right_click_menu_state.position.read();

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
            
            onclick: move |_| {
                right_click_menu_state.close_menu();
            },

            if let DirectoryItem::Directory(_) = directory_item {
                p { button { "Create new directory" } }
                p { button { "Create new file" } }
            }
            p { button { "Delete" } }
            p { button { "Rename" } }
        }
    )
}
