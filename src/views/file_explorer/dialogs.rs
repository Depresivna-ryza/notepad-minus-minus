use std::path::PathBuf;
use std::fs;
use dioxus::prelude::*;

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

    pub fn clear_path(&mut self) {
        self.item_path.set(None);
    }

    pub fn get_operation(&self) -> Option<Operation> {
        self.operation.read().clone()
    }

    pub fn set_operation(&mut self, operation: Operation) {
        self.operation.set(Some(operation));
    }

    pub fn clear_operation(&mut self) {
        self.operation.set(None);
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
            let path = match operation_dialog_handler.get_path() {
                Some(path) => path,
                None => {
                    error_dialog_handler.show("Error path is empty".to_string());
                    operation_dialog_handler.clear_path();
                    operation_dialog_handler.clear_operation();
                    return;
                },
            };

            if new_name().is_empty() {
                error_dialog_handler.show("Name cannot be empty.".to_string());
                operation_dialog_handler.clear_path();
                operation_dialog_handler.clear_operation();
                return;
            }

            match operation_dialog_handler.get_operation() {
                Some(Operation::CreateDirectory) => {
                    if let Err(error) = fs::create_dir(format!("{}/{}", path.to_str().expect(""), new_name)) {
                        error_dialog_handler.show(error.to_string());
                    }
                },
                Some(Operation::CreateFile) => {
                    if let Err(error) = fs::File::create(format!("{}/{}", path.to_str().expect(""), new_name)) {
                        error_dialog_handler.show(error.to_string());
                    }
                },
                Some(Operation::Rename) => {
                    let old_path = path.clone();
                    let mut parent_path = old_path.clone();
                    parent_path.pop();

                    if let Err(error) = fs::rename(old_path.to_str().expect(""), format!("{}/{}", parent_path.to_str().expect(""), new_name)) {
                        error_dialog_handler.show(error.to_string());
                    }
                },
                _ => (),
            }
            
            operation_dialog_handler.clear_path();
            operation_dialog_handler.clear_operation();
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

    let on_submit = {
        let mut operation_dialog_handler = operation_dialog_handler.clone();

        move |_| {
            let path = match operation_dialog_handler.get_path() {
                Some(path) => path,
                None => {
                    error_dialog_handler.show("Error path is empty".to_string());
                    operation_dialog_handler.clear_path();
                    operation_dialog_handler.clear_operation();
                    return;
                },
            };

            match operation_dialog_handler.get_operation() {
                Some(Operation::DeleteDirectory) => {
                    if let Err(error) = fs::remove_dir_all(path.to_str().expect("")) {
                        error_dialog_handler.show(error.to_string());
                    }
                },
                Some(Operation::DeleteFile) => {
                    if let Err(error) = fs::remove_file(path.to_str().expect("")) {
                        error_dialog_handler.show(error.to_string());
                    }
                },
                _ => (),
            }
            
            operation_dialog_handler.clear_path();
            operation_dialog_handler.clear_operation();
        }
    };

    let cancel = {
        let mut operation_dialog_handler = operation_dialog_handler.clone();

        move |_| {
            operation_dialog_handler.clear_path();
            operation_dialog_handler.clear_operation();
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
