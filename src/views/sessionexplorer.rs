use dioxus::prelude::*;

use crate::models::sessions::Sessions;

#[component]
pub fn SessionsExplorer(sessions: Signal<Sessions>) -> Element {
    rsx! {
        div {
            style: "flex: 1; background-color: lightblue;",
            "SessionsExplorer content"
        }
    }
}
