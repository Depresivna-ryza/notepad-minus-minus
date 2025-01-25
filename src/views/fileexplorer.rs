use std::path::PathBuf;
use std::fs;
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
    
    let mut right_click_menu_state = use_context_provider(|| RightClickMenuState {
        is_open: Signal::new(false),
        position: Signal::new((0.0, 0.0)),
    });
    
    let mut state = use_context::<Signal<FileSystem>>();

    let opened_string = match dir.children {
        DirectoryItems::OpenedDirectory(_) => "[v]",
        DirectoryItems::ClosedDirectory => "[>]",
    };

    let path1 = dir.path.clone();
    let path2 = dir.path.clone();

    let style = if state.read().is_focused(&dir.path) {
        "color: red; margin: 5px 0px 5px 20px;"
    } else {
        "color: darkred; margin: 5px 0px 5px 20px;"
    };

    rsx!(
        div {
            style: "color: darkred; margin: 5px 0px 5px 20px;",
            div {
                a {
                    style: "white-space: nowrap;",
                    onclick: move |_| {
                        state.write().find(&path1);
                    },
                    " {opened_string} "
                }
                
                a {
                    style: {style},
                    
                    onclick: move |_| {
                        state.write().change_focus(&path2);
                    },
                    
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
    
    let mut right_click_menu_state = use_context_provider(|| RightClickMenuState {
        is_open: Signal::new(false),
        position: Signal::new((0.0, 0.0)),
    });

    let mut state = use_context::<Signal<FileSystem>>();

    let style = if state.read().is_focused(&file) {
        "color: red; margin: 5px 0px 5px 20px;"
    } else {
        "color: darkred; margin: 5px 0px 5px 20px;"
    };

    let file1 = file.clone();
    let file2 = file.clone();

    rsx!(
        div {
            style: {style},
            
            ondoubleclick: move |_| {
                info!("File clicked: {:?}", file1.clone());

                let mut tabs = use_context::<Signal<Tabs>>();
                tabs.write().open_tab(file1.clone());

            },

            onclick: move |_| {
                state.write().change_focus(&file2);
            },
            
            oncontextmenu: move |event: MouseEvent| {
                right_click_menu_state.handle_right_click(event);
            },
            
            " {file_name}"
        }
        
        if *right_click_menu_state.is_open.read() {
            RightClickMenu { directory_item: DirectoryItem::File(file.clone()) }
        }
    )
}

#[component]
pub fn RightClickMenu(directory_item: DirectoryItem) -> Element {
    let mut right_click_menu_state = use_context::<RightClickMenuState>();
    let mut show_dialog= use_signal(|| false);

    let path = match directory_item {
        DirectoryItem::Directory(ref dir) => dir.path.clone(),
        DirectoryItem::File(ref path_buf) => path_buf.clone(),
    };

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

            if let DirectoryItem::Directory(_) = directory_item {
                p { 
                    button { 
                        class: "option-button",
                        onclick: move |_| { 
                            show_dialog.set(true);
                        },
                        "Create new directory", 
                    } 
                }
                p { button { "Create new file" } }
            }
            p { button { "Delete" } }
            p { button { "Rename" } }

            if *show_dialog.read() {
                NewDirectoryDialog { path, show_dialog }
            }
        }
    )
}

#[component]
pub fn NewDirectoryDialog(path: PathBuf, mut show_dialog: Signal<bool>) -> Element {
    let mut right_click_menu_state = use_context::<RightClickMenuState>();
    let new_directory_name = use_signal(|| String::new());

    let on_input = {
        let mut new_directory_name = new_directory_name.clone();
        move |evt: FormEvent| {
            new_directory_name.set(evt.value().clone());
        }
    };

    let on_submit = {
        let mut new_directory_name = new_directory_name.clone();
        move |_| {
            if !new_directory_name().is_empty() {
                if let Err(error) = fs::create_dir(format!("{}/{}", path.to_str().expect(""), new_directory_name)) {
                    info!("Show Error Dialog: {}", error);
                    return;
                }
                
                new_directory_name.set(String::new());
                show_dialog.set(false);
                right_click_menu_state.close_menu();
            } else {
                println!("Directory name cannot be empty.");
            }
        }
    };

    rsx!(
        div {
            class: "dialog",
            div {
                class: "dialog-content",
                h2 { "Create New Directory" }
                input {
                    class: "directory-input",
                    placeholder: "Enter directory name...",
                    value: "{new_directory_name}",
                    oninput: on_input,
                }
                button {
                    class: "submit-button",
                    onclick: on_submit,
                    "Submit"
                }
            }
        }
    )
}
