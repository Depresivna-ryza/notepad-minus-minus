use std::path::PathBuf;

use crate::models::files::{Dir, DirectoryItem, DirectoryItems, FileSystem};

use rfd::FileDialog;
use tracing::{info};
use dioxus::prelude::*;

#[component]
pub fn FileExplorer(opened_tabs: Signal<Vec<PathBuf>>, current_file: Signal<Option<PathBuf>>) -> Element {
    let mut file_system_state = use_context_provider(|| Signal::new(FileSystem::new()));

    
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
        DirectoryItems::OpenedDirectory(_) => "[-]",
        DirectoryItems::ClosedDirectory => "[+]",
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
                "{dir_name}  {opened_string}" 
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
                info!("File clicked: {:?}", file);
            },
            style: "color: blue; border: 1px solid blue; margin: 5px 0px 5px 20px;",
            "{file_name}"
        }
    )
}