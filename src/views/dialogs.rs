use std::path::PathBuf;
use std::fs;
use std::rc::Rc;
use tracing::info;

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
pub fn NewDirectoryDialog() -> Element {
    let new_directory_name = use_signal(|| String::new());
    let mut operation_dialog_handler = use_context::<OperationDialogHandler>();
    
    let on_input = {
        let mut new_directory_name = new_directory_name.clone();
        move |evt: FormEvent| {
            new_directory_name.set(evt.value().clone());
        }
    };

    let on_submit = {
        let mut new_directory_name = new_directory_name.clone();

        move |_| {

            let path = match operation_dialog_handler.get_path() {
                Some(path) => path,
                None => panic!("Error path is empty"),
            };

            if !new_directory_name().is_empty() {
                if let Err(error) = fs::create_dir(format!("{}/{}", path.to_str().expect(""), new_directory_name)) {
                    info!("Show Error Dialog: {}", error);
                    return;
                }
                
                new_directory_name.set(String::new());
                operation_dialog_handler.clear_path();
                operation_dialog_handler.clear_operation();
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


#[component]
pub fn NewFileDialog() -> Element {
    let new_file_name = use_signal(|| String::new());
    let mut operation_dialog_handler = use_context::<OperationDialogHandler>();
    
    let on_input = {
        let mut new_file_name = new_file_name.clone();
        move |evt: FormEvent| {
            new_file_name.set(evt.value().clone());
        }
    };

    let on_submit = {
        let mut new_file_name = new_file_name.clone();

        move |_| {

            let path = match operation_dialog_handler.get_path() {
                Some(path) => path,
                None => panic!("Error path is empty"),
            };

            if !new_file_name().is_empty() {
                if let Err(error) = fs::File::create(format!("{}/{}", path.to_str().expect(""), new_file_name)) {
                    info!("Show Error Dialog: {}", error);
                    return;
                }
                
                new_file_name.set(String::new());
                operation_dialog_handler.clear_path();
                operation_dialog_handler.clear_operation();
            } else {
                println!("File name cannot be empty.");
            }
        }
    };

    rsx!(
        div {
            class: "dialog",
            div {
                class: "dialog-content",
                h2 { "Create New File" }
                input {
                    class: "file-input",
                    placeholder: "Enter file name...",
                    value: "{new_file_name}",
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

#[component]
pub fn RenameDialog() -> Element {
    let new_name = use_signal(|| String::new());
    let mut operation_dialog_handler = use_context::<OperationDialogHandler>();

    let on_input = {
        let mut new_name = new_name.clone();
        move |evt: FormEvent| {
            new_name.set(evt.value().clone());
        }
    };

    let on_submit = {
        let mut new_name = new_name.clone();

        move |_| {

            let old_path = match operation_dialog_handler.get_path() {
                Some(path) => path,
                None => panic!("Error path is empty"),
            };

            let mut parent_path = old_path.clone();
            parent_path.pop();

            if !new_name().is_empty() {
                if let Err(error) = fs::rename(old_path.to_str().expect(""), format!("{}/{}", parent_path.to_str().expect(""), new_name)) {
                    info!("Show Error Dialog: {}", error);
                    return;
                }
                
                new_name.set(String::new());
                operation_dialog_handler.clear_path();
                operation_dialog_handler.clear_operation();
            } else {
                println!("File name cannot be empty.");
            }
        }
    };

    rsx!(
        div {
            class: "dialog",
            div {
                class: "dialog-content",
                h2 { "Rename" }
                input {
                    class: "file-input",
                    placeholder: "Enter new name...",
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
    )
}

#[component]
pub fn DeleteDirectoryDialog() -> Element {
    let mut operation_dialog_handler = use_context::<OperationDialogHandler>();

    let on_submit = {
        move |_| {

            let path = match operation_dialog_handler.get_path() {
                Some(path) => path,
                None => panic!("Error path is empty"),
            };

            if let Err(error) = fs::remove_dir_all(path.to_str().expect("")) {
                info!("Show Error Dialog: {}", error);
                return;
            }
            
            operation_dialog_handler.clear_path();
            operation_dialog_handler.clear_operation();
        }
    };

    rsx!(
        div {
            class: "dialog",
            div {
                class: "dialog-content",
                h2 { "Delete" }
                p { "Are you sure you want to delete this item?" }
                button {
                    class: "submit-button",
                    onclick: on_submit,
                    "Yes"
                }
            }
        }
    )
}
    
#[component]
pub fn DeleteFileDialog() -> Element {
    let mut operation_dialog_handler = use_context::<OperationDialogHandler>();

    let on_submit = {
        move |_| {

            let path = match operation_dialog_handler.get_path() {
                Some(path) => path,
                None => panic!("Error path is empty"),
            };

            if let Err(error) = fs::remove_file(path.to_str().expect("")) {
                info!("Show Error Dialog: {}", error);
                return;
            }
            
            operation_dialog_handler.clear_path();
            operation_dialog_handler.clear_operation();
        }
    };

    rsx!(
        div {
            class: "dialog",
            div {
                class: "dialog-content",
                h2 { "Delete" }
                p { "Are you sure you want to delete this item?" }
                button {
                    class: "submit-button",
                    onclick: on_submit,
                    "Yes"
                }
            }
        }
    )
}
