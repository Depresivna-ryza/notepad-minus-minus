use dioxus::prelude::*;
use rfd::{AsyncFileDialog, FileDialog};
use crate::views::file_explorer::directory::Directory;
use crate::models::{
    files::{Dir, FileSystem},
    tabs::Tabs,
};

#[component]
pub fn FileExplorer(tabs: Signal<Tabs>) -> Element {
    let mut file_system_state = use_context::<Signal<FileSystem>>();
    use_context_provider(|| tabs);

    let change_root_directory = move |_| async move {
        if let Some(dir_path) = AsyncFileDialog::new().pick_folder().await {
            let mut root_dir = Dir::new(dir_path.path().to_path_buf());
            root_dir.open();
            file_system_state.replace(FileSystem::from(root_dir));
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

            if let Some(dir) = file_system_state.read().root.clone() {
                Directory { dir }
            } else {
                div {
                    "No directory selected"
                }
            }
        }
    }
}
