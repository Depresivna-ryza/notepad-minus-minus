use std::path::PathBuf;

use crate::models::files::{Dir, DirectoryItem, DirectoryItems};

use itertools::chain;
use rfd::FileDialog;
use tracing::{info, warn};
use dioxus::prelude::*;

#[component]
pub fn FileExplorer(opened_tabs: Signal<Vec<PathBuf>>, current_file: Signal<Option<PathBuf>>) -> Element {
    let mut root_directory: Option<Signal<Dir>> = None;
    
    // let open_file = move |_| {
    //     if let Some(file_path) = FileDialog::new().pick_file() {
    //         opened_tabs.push(file_path.clone());
    //         info!("File opened: {:?}", file_path);

    //         current_file.replace(Some(file_path.clone()));
    //         info!("current file changed to: {:?}", file_path);
    //     }
    // };

    let change_root_directory = move |_| {
        if let Some(dir_path) = FileDialog::new().pick_folder() {
            let mut root_dir = Dir::new(dir_path);
            root_dir.open();
            root_directory = Some(use_signal(||root_dir));
        }
    };


    // let dir: Option<Signal<Dir>> = if let Some(root_dir) = root_directory {
    //     Some(use_signal(|| root_dir))
    // } else {
    //     None
    // };

    rsx! {
        div {
            style: "flex: 1; background-color: lightgreen;",

            a {"FileExplorer"}

            button {
                onclick: change_root_directory,
                "Open File"
            }

            if let Some(dir) = root_directory {
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
pub fn Directory(dir: Signal<Dir>) -> Element {
    // let dir = dir.read();

    let dir = dir.read();

    let dir_name = dir.path.file_name().unwrap().to_str().unwrap();

    // let mut child_directories: Vec<Dir> = Vec::new();
    // let mut child_files: Vec<PathBuf> = Vec::new();


    // if let DirectoryItems::OpenedDirectory(ref items) = read.children {
    //     for item in items.iter() {
    //         match item {
    //             DirectoryItem::Directory(dir) => {
    //                 child_directories.push(dir.clone());
    //             }
    //             DirectoryItem::File(file) => {
    //                 child_files.push(file.clone());
    //             }
    //         }
    //     }
    // }


    rsx!(
        div {
            style: "margin-left: 20px;",
            a { 
                onclick: move |_| {
                    dir.open();
                },
                "{dir_name}" 
            }

            for item in dir.children.iter() {
                match item {
                    DirectoryItem::Directory(dir) => {
                        Directory { dir: use_signal(|| dir.clone()) }
                    }
                    
                    DirectoryItem::File(file) => {
                        File { file: file }

                        // let file_name: String = file.file_name().unwrap().to_str().unwrap();

                        
                        // rsx!(div {
                        //     // onclick: move |_| {
                        //     //     info!("File opened: {:?}", file);
                        //     // },
                        //     // style: "margin-left: 20px;",

                        //     a {"123"}
                        // })
                    }
                }
            }

            // for child_dir in child_directories.into_iter() {
            //     // let child_dir = use_signal(|| child_dir);
            //     div {
            //         Directory { dir: use_signal(|| child_dir) }
            //     }
            // }

            // for child_file in child_files.iter() {
            //     div {
            //         style: "margin-left: 20px;",
            //         "{child_file.file_name().unwrap().to_str().unwrap()}"
            //     }
            // }
        }
    )


}

#[component]
pub fn File(file: ReadOnlySignal<PathBuf>) -> Element {
    let file = file.read();
    let file_name = file.file_name().unwrap().to_str().unwrap();

    rsx!(
        div {
            style: "margin-left: 20px;",
            "{file_name}"
        }
    )
}