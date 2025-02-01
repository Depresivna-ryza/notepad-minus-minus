use std::path::PathBuf;
use std::fs;
use tracing::info;

use dioxus::prelude::*;

use crate::models::files::DirectoryItem;

#[derive(Clone, Copy)]
pub struct RightClickMenuState {
    is_open: Signal<bool>,
    position: Signal<(f64, f64)>,
}

impl RightClickMenuState {
    pub fn new() -> Self {
        RightClickMenuState {
            is_open: Signal::new(false),
            position: Signal::new((0.0, 0.0)),
        }
    }

    pub fn handle_right_click(&mut self, event: MouseEvent) {
        event.prevent_default();
        self.is_open.set(true);
        let coordinates = event.client_coordinates();
        self.position.set((coordinates.x, coordinates.y));
    }
    
    pub fn close_menu(&mut self) {
        self.is_open.set(false);
    }

    pub fn is_open(&self) -> bool {
        return *self.is_open.read();
    }
}

#[component]
pub fn RightClickMenu(directory_item: DirectoryItem) -> Element {
    let right_click_menu_state = use_context::<RightClickMenuState>();
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
