use std::path::PathBuf;
use std::fs;
use std::rc::Rc;
use tracing::info;

use dioxus::prelude::*;

#[derive(Clone)]
pub enum Operation {
    CreateDirectory,
    CreateFile,
    Rename,
    Delete,
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
    let mut new_directory_dialog_struct = use_context::<OperationDialogHandler>();
    
    let on_input = {
        let mut new_directory_name = new_directory_name.clone();
        move |evt: FormEvent| {
            new_directory_name.set(evt.value().clone());
        }
    };

    let on_submit = {
        let mut new_directory_name = new_directory_name.clone();

        move |_| {

            let path = match new_directory_dialog_struct.get_path() {
                Some(path) => path,
                None => panic!("Error path is empty"),
            };

            if !new_directory_name().is_empty() {
                info!("path to new dir: {}/{}", path.to_str().expect(""), new_directory_name);
                if let Err(error) = fs::create_dir(format!("{}/{}", path.to_str().expect(""), new_directory_name)) {
                    info!("Show Error Dialog: {}", error);
                    return;
                }
                
                new_directory_name.set(String::new());
                new_directory_dialog_struct.clear_path();
                new_directory_dialog_struct.clear_operation();
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


