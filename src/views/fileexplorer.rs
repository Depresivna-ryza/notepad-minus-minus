use std::path::PathBuf;

use rfd::FileDialog;
use tracing::info;
use dioxus::prelude::*;

#[component]
pub fn FileExplorer(opened_tabs: Signal<Vec<PathBuf>>, current_file: Signal<Option<PathBuf>>) -> Element {
    let open_file = move |_| {
        if let Some(file_path) = FileDialog::new().pick_file() {
            opened_tabs.push(file_path.clone());
            info!("File opened: {:?}", file_path);

            current_file.replace(Some(file_path.clone()));
            info!("current file changed to: {:?}", file_path);
        }
    };

    rsx! {
        div {
            style: "flex: 1; background-color: lightgreen;",
            "FileExplorer"
            button {
                onclick: open_file,
                "Open File"
            }
        }
    }
}