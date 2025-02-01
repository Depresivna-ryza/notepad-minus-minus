use dioxus::prelude::*;
use rfd::FileDialog;
use crate::views::file_explorer::directory::Directory;
use crate::models::{
    files::{Dir, FileSystem},
    tabs::Tabs,
};

#[component]
pub fn FileExplorer(tabs: Signal<Tabs>) -> Element {
    let mut file_system_state = use_context_provider(|| Signal::new(FileSystem::new()));
    use_context_provider(|| tabs);

    let change_root_directory = move |_| {
        if let Some(dir_path) = FileDialog::new().pick_folder() {
            let mut root_dir = Dir::new(dir_path);
            root_dir.open();
            file_system_state.replace(FileSystem::from(root_dir));
        }
    };

    rsx! {
        div {
            style: "flex: 1; background-color: lightgreen; overflow-x: auto; overflow-y: auto;",

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
