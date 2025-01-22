use std::{fs::read_to_string, path::PathBuf};
use crate::models::text::Text;

use dioxus::prelude::*;

#[component]
pub fn Editor(current_file: ReadOnlySignal<Option<PathBuf>>) -> Element {
    let file_content: Option<Text> = match current_file() {
        Some(file_path) => read_to_string(file_path).ok().map(Text::new),
        None => None,
    };

    let Some(file_content) = file_content else {
        return rsx!{
            div {
                style: "background-color: purple; flex: 1; color: red",
                "<NO FILE>"
            }
        }
    };

    rsx!{
        div {
            style: "background-color: purple; flex: 1;",
            ul {
                for text_line in file_content.lines() {
                    li {
                        "{text_line}"
                    }
                }
            }
        }
    }
}


