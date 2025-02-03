use dioxus::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};

use crate::models::file_system::FileSystem;
use crate::views::dialogs::error::ErrorDialogHandler;

#[derive(Clone)]
pub enum Operation {
    CreateDirectory,
    CreateFile,
    DeleteDirectory,
    DeleteFile,
    Rename,
}

#[derive(Clone)]
pub struct OperationDialogHandler {
    item_path: Signal<Option<PathBuf>>,
    operation: Signal<Option<Operation>>,
}

impl Default for OperationDialogHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl OperationDialogHandler {
    pub fn new() -> Self {
        OperationDialogHandler {
            item_path: Signal::new(Option::None),
            operation: Signal::new(None),
        }
    }

    pub fn set_path(&mut self, path: PathBuf) {
        self.item_path.set(Some(path));
    }

    pub fn get_path(&self) -> Option<PathBuf> {
        self.item_path.read().clone()
    }

    pub fn clear(&mut self) {
        self.item_path.set(None);
        self.operation.set(None);
    }

    pub fn get_operation(&self) -> Option<Operation> {
        self.operation.read().clone()
    }

    pub fn set_operation(&mut self, operation: Operation) {
        self.operation.set(Some(operation));
    }

    pub fn is_operation_set(&self) -> bool {
        self.operation.read().is_some()
    }
}

#[component]
pub fn OperationDialog() -> Element {
    let operation_dialog_handler = use_context::<OperationDialogHandler>();

    rsx! {
        div {
            font_family: "JetBrains Mono",
            class: "dialog-overlay",
            div {
                class: "dialog",

                match operation_dialog_handler.get_operation() {
                    Some(Operation::CreateDirectory) | Some(Operation::CreateFile) | Some(Operation::Rename) => rsx!(CreateRenameDialog {}),
                    Some(Operation::DeleteDirectory) | Some(Operation::DeleteFile) => rsx!(DeleteDialog {}),
                    None => rsx!(),
                },
            }
        }
    }
}

#[component]
fn CreateRenameDialog() -> Element {
    let operation_dialog_handler = use_context::<OperationDialogHandler>();
    let mut error_dialog_handler = use_context::<ErrorDialogHandler>();
    let mut file_system = use_context::<Signal<FileSystem>>();

    let new_name = use_signal(String::new);

    let header = match operation_dialog_handler.get_operation() {
        Some(Operation::CreateDirectory) => "Create Directory",
        Some(Operation::CreateFile) => "Create File",
        Some(Operation::Rename) => "Rename",
        _ => "",
    };

    let on_input = {
        let mut new_directory_name = new_name;
        move |evt: FormEvent| {
            new_directory_name.set(evt.value().clone());
        }
    };

    let on_submit = {
        let mut operation_dialog_handler = operation_dialog_handler.clone();

        move |_| {
            let mut new_path = String::new();

            let mut path = match operation_dialog_handler.get_path() {
                Some(path) => path,
                None => {
                    error_dialog_handler.show("Error path is empty".to_string());
                    operation_dialog_handler.clear();
                    return;
                }
            };

            if new_name().is_empty() {
                error_dialog_handler.show("Name cannot be empty.".to_string());
                operation_dialog_handler.clear();
                return;
            }

            match operation_dialog_handler.get_operation() {
                Some(Operation::CreateDirectory) => {
                    new_path = format!(
                        "{}/{}",
                        path.to_str().expect("Path is empty"),
                        new_name.read().as_str()
                    );

                    if let Err(error) = fs::create_dir(&new_path) {
                        error_dialog_handler.show(error.to_string());
                    }
                }
                Some(Operation::CreateFile) => {
                    new_path = format!(
                        "{}/{}",
                        path.to_str().expect("Path is empty"),
                        new_name.read().as_str()
                    );

                    if let Err(error) = fs::File::create(&new_path) {
                        error_dialog_handler.show(error.to_string());
                    }
                }
                Some(Operation::Rename) => {
                    new_path = format!(
                        "{}/{}",
                        path.parent()
                            .expect("Parent path is empty")
                            .to_str()
                            .expect("Path is empty"),
                        new_name.read().as_str()
                    );

                    if let Err(error) = fs::rename(path.to_str().expect(""), &new_path) {
                        error_dialog_handler.show(error.to_string());
                    }
                }
                _ => (),
            }

            // Refresh the file system if the root directory is being renamed
            if matches!(
                operation_dialog_handler.get_operation(),
                Some(Operation::Rename)
            ) && file_system
                .read()
                .get_root()
                .map_or(false, |root| path == *root.get_path())
            {
                file_system.replace(FileSystem::from(Path::new(&new_path)));
                operation_dialog_handler.clear();
                return;
            }

            // Refresh parent if renaming
            if let Some(Operation::Rename) = operation_dialog_handler.get_operation() {
                path.pop();
            }

            // Refresh
            file_system.write().reload_directory(&path);
            operation_dialog_handler.clear();
        }
    };

    let cancel = {
        let mut operation_dialog_handler = operation_dialog_handler.clone();

        move |_| {
            operation_dialog_handler.clear();
        }
    };

    rsx! {
        div {
            class: "dialog-content",
            font_family: "JetBrains Mono",

            h2 {
                font_family: "JetBrains Mono",
                "{ header }"
            }
            input {
                class: "name-input",
                font_family: "JetBrains Mono",
                placeholder: "Enter name...",
                value: "{new_name}",
                oninput: on_input,
            }
            div {
                display: "flex",
                flex_direction: "row",
                div {
                    class: "dialog-button",
                    onclick: on_submit,
                    "Submit"
                }
                div {
                    class: "dialog-button",
                    onclick: cancel,
                    "Cancel"
                }
            }
        }
    }
}

#[component]
fn DeleteDialog() -> Element {
    let operation_dialog_handler = use_context::<OperationDialogHandler>();
    let mut error_dialog_handler = use_context::<ErrorDialogHandler>();
    let mut file_system = use_context::<Signal<FileSystem>>();

    let on_submit = {
        let mut operation_dialog_handler = operation_dialog_handler.clone();

        move |_| {
            let mut path = match operation_dialog_handler.get_path() {
                Some(path) => path,
                None => {
                    error_dialog_handler.show("Error path is empty".to_string());
                    operation_dialog_handler.clear();
                    return;
                }
            };

            match operation_dialog_handler.get_operation() {
                Some(Operation::DeleteDirectory) => {
                    if let Err(error) = fs::remove_dir_all(path.to_str().expect("Path is empty")) {
                        error_dialog_handler.show(error.to_string());
                    }
                }
                Some(Operation::DeleteFile) => {
                    if let Err(error) = fs::remove_file(path.to_str().expect("Path is empty")) {
                        error_dialog_handler.show(error.to_string());
                    }
                }
                _ => (),
            }

            // Refresh the file system if the root directory is being deleted
            if file_system
                .read()
                .get_root()
                .map_or(false, |root| path == *root.get_path())
            {
                file_system.replace(FileSystem::new());
                operation_dialog_handler.clear();
                return;
            }

            // Refresh
            path.pop();
            file_system.write().reload_directory(&path);
            operation_dialog_handler.clear();
        }
    };

    let cancel = {
        let mut operation_dialog_handler = operation_dialog_handler.clone();

        move |_| {
            operation_dialog_handler.clear();
        }
    };

    rsx! {
        div {
            font_family: "JetBrains Mono",
            flex_direction: "row",
            p {
                font_family: "JetBrains Mono",
                "Are you sure you want to delete this item?"
            }

            div {
                class: "dialog-content",
                display: "flex",

                div {
                    class: "dialog-button",
                    onclick: on_submit,
                    "Yes"
                }
                div {
                    class: "dialog-button",
                    onclick: cancel,
                    "Cancel"
                }
            }
        }
    }
}
