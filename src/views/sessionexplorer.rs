use dioxus::prelude::*;

#[component]
pub fn SessionsExplorer() -> Element {
    rsx! {
        div {
            style: "flex: 1; background-color: lightblue;",
            "SessionsExplorer content"
        }
    }
}
