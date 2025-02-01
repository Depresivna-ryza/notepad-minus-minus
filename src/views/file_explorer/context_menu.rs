
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
    let mut right_click_menu_state = use_context::<RightClickMenuState>();
    let mut show_dialog= use_signal(|| false);

    let path = match directory_item {
        DirectoryItem::Directory(ref dir) => dir.path.clone(),
        DirectoryItem::File(ref path_buf) => path_buf.clone(),
    };

    let menu_position = right_click_menu_state.position.read();

    rsx!(
        div {
            class: "right-click-menu",

            style: "
                top: {menu_position.1}px;
                left: {menu_position.0}px;
            ",

            onclick: move |_| {
                right_click_menu_state.close_menu();
            },

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
        }
    )
}
