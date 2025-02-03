use crate::models::file_system::FileSystem;
use crate::models::file_system::FileSystemItem;
use crate::views::dialogs::fs_operations::{Operation, OperationDialogHandler};
use dioxus::prelude::*;

#[derive(Clone)]
pub struct RightClickMenuHandler {
    is_open: bool,
    position: (f64, f64),
    from_fs_item: Option<FileSystemItem>,
}

impl Default for RightClickMenuHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl RightClickMenuHandler {
    pub fn new() -> Self {
        RightClickMenuHandler {
            is_open: false,
            position: (0.0, 0.0),
            from_fs_item: None,
        }
    }

    pub fn handle_right_click(&mut self, event: MouseEvent) {
        event.prevent_default();
        self.is_open = true;
        let coordinates = event.client_coordinates();
        self.position = (coordinates.x, coordinates.y);
    }

    pub fn close_menu(&mut self) {
        self.is_open = false;
    }

    pub fn is_open(&self) -> bool {
        self.is_open
    }

    pub fn set_fs_item(&mut self, fs_item: FileSystemItem) {
        self.from_fs_item = Some(fs_item);
    }

    pub fn get_fs_item(&self) -> Option<FileSystemItem> {
        self.from_fs_item.clone()
    }

    pub fn get_posion(&self) -> (f64, f64) {
        self.position
    }
}

#[component]
pub fn RightClickMenu(fs_item: FileSystemItem) -> Element {
    let mut right_click_menu_state = use_context::<Signal<RightClickMenuHandler>>();
    let mut focus_state = use_context::<Signal<FileSystem>>();
    let operation_dialog_handler = use_context::<OperationDialogHandler>();

    let mut button_pressed = use_signal(|| false);

    let menu_position = right_click_menu_state.read().get_posion();

    let path = match fs_item {
        FileSystemItem::Directory(ref dir) => dir.get_path().clone(),
        FileSystemItem::File(ref path_buf) => path_buf.clone(),
    };

    let create_directory = {
        let path = path.clone();
        let mut operation_dialog_handler = operation_dialog_handler.clone();
        let mut right_click_menu_state = right_click_menu_state;

        move |_| {
            operation_dialog_handler.set_operation(Operation::CreateDirectory);
            operation_dialog_handler.set_path(path.clone());
            right_click_menu_state.write().close_menu();
        }
    };

    let create_file = {
        let path = path.clone();
        let mut operation_dialog_handler = operation_dialog_handler.clone();
        let mut right_click_menu_state = right_click_menu_state;

        move |_| {
            operation_dialog_handler.set_operation(Operation::CreateFile);
            operation_dialog_handler.set_path(path.clone());
            right_click_menu_state.write().close_menu();
        }
    };

    let delete_dir = {
        let path = path.clone();
        let mut operation_dialog_handler = operation_dialog_handler.clone();
        let mut right_click_menu_state = right_click_menu_state;

        move |_| {
            operation_dialog_handler.set_operation(Operation::DeleteDirectory);
            operation_dialog_handler.set_path(path.clone());
            right_click_menu_state.write().close_menu();
        }
    };

    let delete_file = {
        let path = path.clone();
        let mut operation_dialog_handler = operation_dialog_handler.clone();
        let mut right_click_menu_state = right_click_menu_state;

        move |_| {
            operation_dialog_handler.set_operation(Operation::DeleteFile);
            operation_dialog_handler.set_path(path.clone());
            right_click_menu_state.write().close_menu();
        }
    };

    let rename = {
        let path = path.clone();
        let mut operation_dialog_handler = operation_dialog_handler.clone();
        let mut right_click_menu_state = right_click_menu_state;

        move |_| {
            operation_dialog_handler.set_operation(Operation::Rename);
            operation_dialog_handler.set_path(path.clone());
            right_click_menu_state.write().close_menu();
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
                    right_click_menu_state.write().close_menu();
                    focus_state.write().clear_focus();
                }
            },

            div {
                class: "dialog-content",
                onmousedown: move |_| button_pressed.set(true),

                match fs_item {
                    FileSystemItem::Directory(_) => rsx!(
                        div {
                            class: "option-button",
                            onclick: create_directory,
                            "Create new directory",
                        }
                        div {
                            class: "option-button",
                            onclick: create_file,
                            "Create new file"
                        }
                        div {
                            class: "option-button",
                            onclick: delete_dir,
                            "Delete"
                        }
                    ),
                    FileSystemItem::File(_) => rsx!(
                        div {
                            class: "option-button",
                            onclick: delete_file,
                            "Delete"
                        }
                    ),
                }
                div {
                    class: "option-button",
                    onclick: rename,
                    "Rename"
                }
            }
        }
    )
}
