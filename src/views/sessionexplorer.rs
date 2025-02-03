use dioxus::prelude::*;

#[component]
pub fn SessionsExplorer() -> Element {
    rsx! {
        div {
            style: "display: flex; flex: 1; background-color: rgb(53, 40, 40);  font-family: JetBrains Mono; font-size: 20px; color: white;
                    justify-content: center; align-items: center; width: 100%;",
                "Sessions"
        }
    }
}
