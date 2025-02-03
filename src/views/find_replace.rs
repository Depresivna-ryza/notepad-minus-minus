use dioxus::prelude::*;

use crate::models::tabs::Tabs;

#[component]
pub fn FindReplace(tabs: Signal<Tabs>) -> Element {
    let mut needle = use_signal(|| "".to_string());
    let mut search_idx: Signal<Option<usize>> = use_signal(|| None);
    let mut case_sensitive = use_signal(|| false);

    rsx! {
        div {
            style: "flex: 1; background-color: rgb(59, 53, 45); display: flex; flex-direction: column;
                    font-family: JetBrains Mono; font-size: 20px; color: white; align-items: center;",

            div {
                style: "display: flex; width: 100%; justify-content: center; align-items: center; padding: 10px;
                    ",
                "Find"
            }

            input {
                type: "text",
                style: "width: 100%; padding: 10px; font-family: JetBrains Mono;
                        font-size: 20px; background-color: #1e1e1e; color: white; border: none; outline: none;",
                value: needle(),
                oninput: move |e| {
                    needle.set(e.value());
                }
            }

            div {
                style: "display: flex; flex-direction: row; width: 100%; justify-content: center;",

                input {
                    class: "custom-checkbox",
                    type: "checkbox",
                    checked: case_sensitive(),
                    onchange: move |e| {
                        case_sensitive.set(e.checked());
                    }
                }

                label {
                    style: "margin-left: 5px; font-weight: 100; font-size: 17px; text-align: center;",
                    "Case sensitive"
                }
            }

            div {
                style: "display: flex; flex-direction: row; width: 100%;",
                button {
                    class: "custom-button",
                    onclick: move |_| {
                        if needle.peek().is_empty() {
                            return;
                        }

                        tabs.write().get_current_file_mut().map(|f| {
                            let mut res = f.find_and_select(search_idx.peek().unwrap_or(0), needle(), false, case_sensitive());
                            if res.is_none() && search_idx.peek().unwrap_or(0) != 0 {
                                res = f.find_and_select(0, needle(), true, case_sensitive());
                            }

                            search_idx.set(res.map(|r| r + 1));
                        });
                    },
                    "Get next"
                }
                div {
                    style: "width: 1px; background-color: rgba(73, 173, 255, 0.15); height: 100%;",
                }

                button {
                    class: "custom-button",
                    onclick: move |_| {
                        if needle.peek().is_empty() {
                            return;
                        }

                        if let Some(f) = tabs.write().get_current_file_mut() {
                            let len = f.rope.len_chars();
                            let res = f.find_and_select(search_idx.peek().unwrap_or(len - 1), needle(), true, case_sensitive());
                            search_idx.set(res.and_then(|r| r.checked_sub(1)));
                        };
                    },
                    "Get previous"
                }
            }
         }
    }
}
