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
                    style: "display: flex; justify-content: center; align-items: center;",
                    "No file selected"
                }
            }
        }
    };

    let history = file.event_history.clone();
    let history_idx = 1.max(file.history_idx) - 1;

    rsx! {
        div {
            style: "flex: 1; background-color: orange; overflow-x: auto; overflow-y: auto; display: flex; flex-direction: column;",
            a {"History"}

            for (i, event) in history.iter().enumerate().rev() {
                HistoryLine {
                    tabs: tabs, 
                    ln: i,
                    event: event.clone(), 
                    current_history_idx: history_idx
                }
            }

        }
    }
}

#[component]
pub fn HistoryLine(tabs: Signal<Tabs>, ln: usize, event: HistoryEvent, current_history_idx: usize) -> Element {
    let str = format!("{}", event);
    rsx! {
        div {
            style: "border: 1px solid black; margin: 5px; ".to_string() + 
            match ln == current_history_idx {
                true => "background-color: green",
                false => ""
            },
            onclick: move |_| {
                tabs.write().get_current_file_mut().map(|f| {
                    f.go_to_history_idx(ln + 1);
                });
            },
            "{str}"
        }
    }
}