
use dioxus::prelude::*;
use crate::models::file_system::FileSystemItem;
use crate::views::dialogs::fs_operations::{Operation, OperationDialogHandler};
use crate::models::file_system::FileSystem;

#[derive(Clone, Copy)]
pub struct RightClickMenuHandler {
    is_open: Signal<bool>,
    position: Signal<(f64, f64)>,
}

impl RightClickMenuHandler {
    pub fn new() -> Self {
        RightClickMenuHandler {
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
pub fn RightClickMenu(fs_item: FileSystemItem) -> Element {
    let mut right_click_menu_state = use_context::<RightClickMenuHandler>();
    let mut focus_state = use_context::<Signal<FileSystem>>();
    let operation_dialog_handler = use_context::<OperationDialogHandler>();

    let mut button_pressed = use_signal(|| false);

    let menu_position = right_click_menu_state.position.read();

    let path = match fs_item {
        FileSystemItem::Directory(ref dir) => dir.get_path().clone(),
        FileSystemItem::File(ref path_buf) => path_buf.clone(),
    };

    let create_directory = {
        let path = path.clone();
        let mut operation_dialog_handler = operation_dialog_handler.clone();

        move |_| {
            operation_dialog_handler.set_operation(Operation::CreateDirectory);
            operation_dialog_handler.set_path(path.clone());
            right_click_menu_state.close_menu();
        }
    };

    let create_file = {
        let path = path.clone();
        let mut operation_dialog_handler = operation_dialog_handler.clone();

        move |_| {
            operation_dialog_handler.set_operation(Operation::CreateFile);
            operation_dialog_handler.set_path(path.clone());
            right_click_menu_state.close_menu();
        }
    };

    let delete_dir = {
        let path = path.clone();
        let mut operation_dialog_handler = operation_dialog_handler.clone();

        move |_| {
            operation_dialog_handler.set_operation(Operation::DeleteDirectory);
            operation_dialog_handler.set_path(path.clone());
            right_click_menu_state.close_menu();
        }
    };

    let delete_file = {
        let path = path.clone();
        let mut operation_dialog_handler = operation_dialog_handler.clone();

        move |_| {
            operation_dialog_handler.set_operation(Operation::DeleteFile);
            operation_dialog_handler.set_path(path.clone());
            right_click_menu_state.close_menu();
        }
    };

    let rename = {
        let path = path.clone();
        let mut operation_dialog_handler = operation_dialog_handler.clone();

        move |_| {
            operation_dialog_handler.set_operation(Operation::Rename);
            operation_dialog_handler.set_path(path.clone());
            right_click_menu_state.close_menu();
        }
    };

    rsx!(
        div {
            class: "right-click-menu",
            tabindex: 0,
            style: "
                top: {menu_position.1}px;
                left: {menu_position.0}px;
            ",
            
            onmounted: move |e| async move {
                let _ = e.data().as_ref().set_focus(true).await;
            },

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

                match fs_item {
                    FileSystemItem::Directory(_) => rsx!(
                        p { 
                            button { 
                                class: "option-button",
                                onclick: create_directory,
                                "Create new directory", 
                            } 
                        }
                        p { 
                            button { 
                                class: "option-button",
                                onclick: create_file,
                                "Create new file" 
                            } 
                        }
                        p { 
                            button { 
                                class: "option-button",
                                onclick: delete_dir,
                                "Delete" 
                            } 
                        }
                    ),
                    FileSystemItem::File(_) => rsx!(
                        p {
                            button {
                                class: "option-button",
                                onclick: delete_file,
                                "Delete"
                            }
                        }
                    ),
                }
                p { 
                    button { 
                        class: "option-button",
                        onclick: rename,
                        "Rename" 
                    } 
                }
            }
        }
    )
}
