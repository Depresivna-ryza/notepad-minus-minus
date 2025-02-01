use dioxus::prelude::*;

use crate::models::{event::HistoryEvent, tabs::Tabs};




#[component]
pub fn EditHistory(tabs: Signal<Tabs>) -> Element {

    let f = use_memo(move || tabs.read().get_current_file());

    let Some(ref file) = *f.read() else {
        return rsx! {
            div {
                style: "flex: 1; background-color: orange; overflow-x: auto; overflow-y: auto;",
                div {
                    style: "display: flex; justify-content: center; align-items: center; height: 100%",
                    "No file selected"
                }
            }
        }
    };

    let history = file.event_history.clone();

    rsx! {
        div {
            style: "flex: 1; background-color: orange; overflow-x: auto; overflow-y: auto;",

            a {"History of changes"}

            div {
                style: "display: flex; flex-direction: column;",

                for (i, event) in history.iter().enumerate() {
                    HistoryLine {tabs: tabs, ln: i, event: event.clone()}
                }
            }

        }
    }
}

#[component]
pub fn HistoryLine(tabs: Signal<Tabs>, ln: usize, event: HistoryEvent) -> Element {
    let str = format!("[#{}]: {:?}", ln, event);
    rsx! {
        div {
            "{str}"
        }
    }
}