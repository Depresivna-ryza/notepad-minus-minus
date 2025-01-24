use std::path::PathBuf;

use crate::models::{
    files::{Dir, DirectoryItem, DirectoryItems, FileSystem},
    tabs::Tabs,
};

use dioxus::prelude::*;
use rfd::FileDialog;
use tracing::info;

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

#[component]
pub fn Directory(dir: Dir) -> Element {
    let dir_name = dir.path.file_name().unwrap().to_str().unwrap();

    let opened_string = match dir.children {
        DirectoryItems::OpenedDirectory(_) => "[v]",
        DirectoryItems::ClosedDirectory => "[>]",
    };

    rsx!(
        div {
            style: "color: darkred; border: 1px solid darkred; margin: 5px 0px 5px 20px;",
            a {
                style: "white-space: nowrap;",
                onclick: move |_| {
                    info!("File clicked: {:?}", dir.path);

                    let mut state = use_context::<Signal<FileSystem>>();
                    state.write().find(&dir.path);
                },
                " {opened_string} {dir_name} "
            }
            if let DirectoryItems::OpenedDirectory(dir_items) = dir.children {
                for item in dir_items.iter() {
                    if let DirectoryItem::Directory(dir) = item {
                        Directory { dir: dir.clone()}
                    }
                }

                for item in dir_items.iter() {
                    if let DirectoryItem::File(file) = item {
                        File { file: file.clone() }
                    }
                }
            }
        }
    )
}

#[component]
pub fn File(file: PathBuf) -> Element {
    let file_name = file.file_name().unwrap().to_str().unwrap();

    rsx!(
        div {
            onclick: move |_| {
                info!("File clicked: {:?}", file.clone());

                let mut tabs = use_context::<Signal<Tabs>>();
                tabs.write().open_tab(file.clone());

            },
            style: "color: blue; border: 1px solid blue; margin: 5px 0px 5px 20px;",
            "{file_name}"
        }
    )
}
