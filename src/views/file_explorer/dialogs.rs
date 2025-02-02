use std::path::PathBuf;
use std::fs;
use dioxus::prelude::*;

use crate::models::files::{Dir, FileSystem};

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
            class: "dialog",

            match operation_dialog_handler.get_operation() {
                Some(Operation::CreateDirectory) | Some(Operation::CreateFile) | Some(Operation::Rename) => rsx!(CreateRenameDialog {}),
                Some(Operation::DeleteDirectory) | Some(Operation::DeleteFile) => rsx!(DeleteDialog {}),
                None => rsx!(),
            },
        }
    }
}

#[component]
pub fn CreateRenameDialog() -> Element {
    let mut operation_dialog_handler = use_context::<OperationDialogHandler>();
    let mut error_dialog_handler = use_context::<ErrorDialogHandler>();
    let mut file_system = use_context::<Signal<FileSystem>>();

    let new_name = use_signal(|| String::new());

    let header = match operation_dialog_handler.get_operation() {
        Some(Operation::CreateDirectory) => "Create Directory",
        Some(Operation::CreateFile) => "Create File",
        Some(Operation::Rename) => "Rename",
        _ => "",
    };

    let on_input = {
        let mut new_directory_name = new_name.clone();
        move |evt: FormEvent| {
            new_directory_name.set(evt.value().clone());
        }
    };

    let on_submit = {
        let new_name = new_name.clone();

        move |_| {
            let mut new_path = String::new();

            let mut path = match operation_dialog_handler.get_path() {
                Some(path) => path,
                None => {
                    error_dialog_handler.show("Error path is empty".to_string());
                    operation_dialog_handler.clear();
                    return;
                },
            };

            if new_name().is_empty() {
                error_dialog_handler.show("Name cannot be empty.".to_string());
                operation_dialog_handler.clear();
                return;
            }

            match operation_dialog_handler.get_operation() {
                Some(Operation::CreateDirectory) => {
                    new_path = format!("{}/{}", path.to_str().expect("Path is empty"), new_name.read().as_str());

                    if let Err(error) = fs::create_dir(&new_path) {
                        error_dialog_handler.show(error.to_string());
                    }
                },
                Some(Operation::CreateFile) => {
                    new_path = format!("{}/{}", path.to_str().expect("Path is empty"), new_name.read().as_str());

                    if let Err(error) = fs::File::create(&new_path) {
                        error_dialog_handler.show(error.to_string());
                    }
                },
                Some(Operation::Rename) => {
                    new_path = format!("{}/{}", path.parent().expect("Parent path is empty").to_str().expect("Path is empty"), new_name.read().as_str());

                    if let Err(error) = fs::rename(path.to_str().expect(""), &new_path) {
                        error_dialog_handler.show(error.to_string());
                    }

                },
                _ => (),
            }

            // Refresh the file system if the root directory is being renamed
            if matches!(operation_dialog_handler.get_operation(), Some(Operation::Rename)) && file_system.read().get_root_path().map_or(false, |root_path| path == *root_path) {
                file_system.replace(FileSystem::from(Dir::new(PathBuf::from(&new_path))));
                operation_dialog_handler.clear();
                return;
            }
            
            // Refresh parent if renaming
            if let Some(Operation::Rename) = operation_dialog_handler.get_operation() {
                path.pop();
            }

            // Refresh
            file_system.write().find(&path);
            file_system.write().find(&path);
            
            operation_dialog_handler.clear();
        }
    };

    rsx! {
        div {
            class: "dialog-content",
            h2 { { header } }
            input {
                class: "name-input",
                placeholder: "Enter name...",
                value: "{new_name}",
                oninput: on_input,
            }
            button {
                class: "submit-button",
                onclick: on_submit,
                "Submit"
            }
        }
    }
}

#[component]
pub fn DeleteDialog() -> Element {
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
                },
            };

            match operation_dialog_handler.get_operation() {
                Some(Operation::DeleteDirectory) => {
                    if let Err(error) = fs::remove_dir_all(path.to_str().expect("Path is empty")) {
                        error_dialog_handler.show(error.to_string());
                    }
                },
                Some(Operation::DeleteFile) => {
                    if let Err(error) = fs::remove_file(path.to_str().expect("Path is empty")) {
                        error_dialog_handler.show(error.to_string());
                    }
                },
                _ => (),
            }
            
            // Refresh the file system if the root directory is being deleted
            if file_system.read().get_root_path().map_or(false, |root_path| path == *root_path) {
                file_system.replace(FileSystem::new());
                operation_dialog_handler.clear();
                return;
            }

            // Refresh
            path.pop();
            file_system.write().find(&path);
            file_system.write().find(&path);
            
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
            p { "Are you sure you want to delete this item?" }
            button {
                class: "submit-button",
                onclick: on_submit,
                "Yes"
            }
            button { 
                class: "cancel-button",
                onclick: cancel,
                "Cancel"
            }
        }
    }
}


#[derive(Clone)]
pub struct ErrorDialogHandler {
    show: Signal<bool>,
    message: Signal<Option<String>>,
}

impl ErrorDialogHandler {
    pub fn new() -> Self {
        Self {
            show: Signal::new(false),
            message: Signal::new(None),
        }
    }

    pub fn show(&mut self, message: String) {
        self.message.set(Some(message));
        self.show.set(true);
    }

    pub fn get_message(&self) -> String {
        self.message.read().clone().unwrap_or_else(|| String::new())
    }

    pub fn is_shown(&self) -> bool {
        *self.show.read()
    }

    pub fn hide(&mut self) {
        self.show.set(false);
    }
}

#[component]
pub fn ErrorDialog() -> Element {
    let error_dialog_handler = use_context::<ErrorDialogHandler>();

    let close = {
        let mut error_dialog_handler = error_dialog_handler.clone();
        move |_| {
            error_dialog_handler.hide();
        }
    };

    rsx! {
        div {
            class: "dialog",
            div {
                class: "dialog-content",
                p { "An error occurred." }
                p { { error_dialog_handler.get_message() } }
                button { 
                    class: "cancel-button",
                    onclick: close,
                    "Close"
                }
            }
        }
    }
}
