use dioxus::prelude::*;
use rfd::AsyncFileDialog;
use crate::views::file_explorer::directory::DirectoryComponent;
use crate::models::tabs::Tabs;
use crate::views::dialogs::fs_operations::{OperationDialogHandler, OperationDialog};
use crate::models::file_system::FileSystem;

#[component]
pub fn FileExplorer(tabs: Signal<Tabs>) -> Element {
    use_context_provider(|| tabs);
    let operation_dialog_handler = use_context_provider(|| OperationDialogHandler::new());

    let mut file_system = use_context_provider(|| Signal::new(FileSystem::new()));

    let change_root_directory = move |_| async move {
        if let Some(dir_path) = AsyncFileDialog::new().pick_folder().await {
            file_system.replace(FileSystem::from(dir_path.path()));
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

            if let Some(directory) = file_system.read().get_root() {
                DirectoryComponent { path: directory.get_path().clone() }
            } else {
                div {
                    "No directory selected"
                }
            }
        }

        if operation_dialog_handler.is_operation_set() {
            OperationDialog {}
        }
    }
}
