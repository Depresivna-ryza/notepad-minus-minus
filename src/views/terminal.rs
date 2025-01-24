use dioxus::prelude::*;


#[component]
pub fn Terminal(hidden: Signal<bool>) -> Element {
    print!("Terminal hidden: {}", hidden.peek());
    rsx! {
        div {
            hidden: hidden,
            style: "background-color: lightgreen; overflow-x: auto; overflow-y: auto; height: 200px;",
            "Terminal"
        }
    }
}