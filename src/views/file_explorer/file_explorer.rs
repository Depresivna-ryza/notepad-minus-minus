use crate::models::file_system::FileSystem;
use crate::models::tabs::Tabs;
use crate::views::dialogs::fs_operations::{OperationDialog, OperationDialogHandler};
use crate::views::file_explorer::directory::DirectoryComponent;
use dioxus::prelude::*;
use rfd::AsyncFileDialog;

use std::time::Duration as duration;

#[component]
pub fn FileExplorer(tabs: Signal<Tabs>) -> Element {
    use_context_provider(|| tabs);
    let operation_dialog_handler = use_context::<OperationDialogHandler>();

    let mut file_system = use_context::<Signal<FileSystem>>();

    let change_root_directory = move |_| async move {
        if let Some(dir_path) = AsyncFileDialog::new().pick_folder().await {
            file_system.replace(FileSystem::from(dir_path.path()));
        }
    };

    use_future(move || async move {
        loop {
            tokio::time::sleep(duration::from_secs(1)).await;
            file_system.write().reload();
        }
    });

    rsx! {
        div {
            class: "file-explorer",

            div {
                style: "display: flex; width: 100%; align-items: center;
                        padding-left: 10px; height: 35px; background-color: rgb(26, 28, 48);",
                "FileExplorer"
            }

            div {
                style: "height: 1px; background-color: rgb(59, 63, 105); width: 100%;",
            }

            div {
                style: "width: 100%; height: 100%; display: flex; overflow-y: auto; color: white;
                        font-family: JetBrains Mono; font-size: 14px;",
                if let Some(directory) = file_system.read().get_root() {
                    DirectoryComponent { path: directory.get_path().clone() }
                } else {
                    div {
                        style: "width: 100%; height: 100%; display: flex; justify-content: center; align-items: center; color: rgb(175, 175, 175)",
                        "No directory selected"
                    }
                }
            }
        }

        if operation_dialog_handler.is_operation_set() {
            OperationDialog {}
        }

        div {
            class: "choose-directory-btn",
            onclick: change_root_directory,
            "Change root directory"
        }
    }
}
