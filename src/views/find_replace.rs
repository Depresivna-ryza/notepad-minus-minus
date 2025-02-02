use dioxus::prelude::*;

use crate::models::tabs::Tabs;

#[component]
pub fn FindReplace(tabs: Signal<Tabs>) -> Element {

    let f = use_memo(move || tabs.read().get_current_file());

    let mut needle = use_signal(|| "".to_string());
    let mut search_idx: Signal<Option<usize>> = use_signal(|| None);
    let mut case_sensitive = use_signal(|| false);

    rsx!{
        div { 
            style: "flex: 1; background-color: lightgreen; display: flex; flex-direction: column;",

            a {"Find"}

            input { 
                type: "text",
                value: needle(),
                oninput: move |e| {
                    needle.set(e.value());
                }
            }

            div {  
                style: "display: flex; flex-direction: row;",

                input {
                    type: "checkbox",
                    checked: case_sensitive(),
                    onchange: move |e| {
                        case_sensitive.set(e.checked());
                    }
                }

                label {
                    "Case sensitive"
                }
            }

            button {
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

            button {
                onclick: move |_| {
                    if needle.peek().is_empty() {
                        return;
                    }

                    tabs.write().get_current_file_mut().map(|f| {
                        let len = f.rope.len_chars();
                        let res = f.find_and_select(search_idx.peek().unwrap_or(len - 1) , needle(), true, case_sensitive());
                        search_idx.set(res.and_then(|r| r.checked_sub(1)));
                    });
                },
                "Get previous"
            }
         }
    }
}