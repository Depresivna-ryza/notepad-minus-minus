use dioxus::prelude::*;
use rfd::{AsyncFileDialog, FileDialog};
use crate::views::file_explorer::directory::Directory;
use crate::models::{
    files::{Dir, FileSystem},
    tabs::Tabs,
};
use crate::views::file_explorer::dialogs::{OperationDialogHandler, OperationDialog, ErrorDialogHandler, ErrorDialog};

#[component]
pub fn FileExplorer(tabs: Signal<Tabs>) -> Element {
    use_context_provider(|| tabs);
    let operation_dialog_handler = use_context_provider(|| OperationDialogHandler::new());
    let error_dialog_handler = use_context_provider(|| ErrorDialogHandler::new());

    let mut file_system = use_context_provider(|| Signal::new(FileSystem::new()));

    let change_root_directory = move |_| async move {
        if let Some(dir_path) = AsyncFileDialog::new().pick_folder().await {
            let mut root_dir = Dir::new(dir_path.path().to_path_buf());
            root_dir.open();
            file_system.replace(FileSystem::from(root_dir));
        }
    };

    rsx! {
        div {
            class: "file-explorer",

            a {"FileExplorer"}

            button {
                onclick: change_root_directory,
                "Change root directory"
            }

            if let Some(dir) = file_system.read().root.clone() {
                Directory { dir }
            } else {
                div {
                    "No directory selected"
                }
            }
        }

        if operation_dialog_handler.is_operation_set() {
            OperationDialog {}
        }

        if error_dialog_handler.is_shown() {
            ErrorDialog {}
        }

    }
}
