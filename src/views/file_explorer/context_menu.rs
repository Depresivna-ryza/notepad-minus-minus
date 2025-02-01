use dioxus::prelude::*;
use tracing::info;

use crate::models::files::DirectoryItem;
use crate::views::dialogs::NewDirectoryDialogStruct;
use crate::models::files::FileSystem;

use std::time::Duration;
use settimeout::set_timeout;

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
    let mut right_click_menu_state = use_context::<RightClickMenuState>();
    let mut focus_state = use_context::<Signal<FileSystem>>();
    let mut new_directory_dialog_struct = use_context::<NewDirectoryDialogStruct>();

    let menu_position = right_click_menu_state.position.read();

    let path = match directory_item {
        DirectoryItem::Directory(ref dir) => dir.path.clone(),
        DirectoryItem::File(ref path_buf) => path_buf.clone(),
    };

    let mut button_pressed = use_signal(|| false);

    rsx!(
        div {
            onmounted: move |e| {
                e.data().as_ref().set_focus(true);
            },
            
            class: "right-click-menu",

            style: "
                top: {menu_position.1}px;
                left: {menu_position.0}px;
            ",

            tabindex: 0,
            
            onfocusout: move |_| {
                if *button_pressed.read() {
                    button_pressed.set(false);
                } else {
                    right_click_menu_state.close_menu();
                    focus_state.write().clear_focus();
                }
            },
            
            div {
                onmousedown: move |_| button_pressed.set(true),

                if let DirectoryItem::Directory(_) = directory_item {
                    p { 
                        button { 
                            class: "option-button",
                            onclick: move |_| { 
                                info!("path: {:?}", path.clone());
                                new_directory_dialog_struct.set_path(path.clone());
                                right_click_menu_state.close_menu();
                            },
                            "Create new directory", 
                        } 
                    }
                    p { 
                        button { 
                            "Create new file" 
                        } 
                    }
                }
                p { button { "Delete" } }
                p { button { "Rename" } }
            }
        }
    )
}
