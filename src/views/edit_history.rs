use dioxus::prelude::*;

use crate::models::{historyevent::HistoryEvent, tabs::Tabs};

#[component]
pub fn EditHistory(tabs: Signal<Tabs>) -> Element {
    let f = use_memo(move || tabs.read().get_current_file());

    let Some(ref file) = *f.read() else {
        return rsx! {
            div {
                style: "flex: 1; background-color: rgb(124, 123, 65); overflow: hidden;
                        width: 100%; display: flex; color: white; justify-content: center; align-items: center; font-size: 20px; font-family: JetBrains Mono;",
                div {
                    style: "display: flex; justify-content: center; align-items: center; color: rgb(204, 204, 204)",
                    "No file selected"
                }
            }
        };
    };

    let history = file.event_history.clone();
    let history_idx = 1.max(file.history_idx) - 1;

    rsx! {
        div {
            style:"flex: 1; background-color: rgb(124, 123, 65); overflow: hidden; flex-direction: column;
                   width: 100%; display: flex; color: white; justify-content: center; align-items: center; font-size: 20px; font-family: JetBrains Mono;",
            div {
                style: "display: flex; justify-content: center; align-items: center; width: 100%; padding: 7px",
                "History"
            }
            div {
                // divider
                style: "height: 3px; background-color: rgba(105, 211, 238, 0.43); width: 100%;",
            }
            div {
                style: "display: flex; flex-direction: column; width: 100%;
                        align-items: center; overflow-y: auto; flex: 1",
                for (i, event) in history.iter().enumerate().rev() {
                    HistoryLine {
                        tabs: tabs,
                        ln: i,
                        event: event.clone(),
                        current_history_idx: history_idx
                    }
                    div {
                        //divider
                        style: "height: 1px; background-color: rgba(206, 206, 206, 0.35); width: 90%;",
                    }
                }
            }

        }
    }
}

#[component]
pub fn HistoryLine(
    tabs: Signal<Tabs>,
    ln: usize,
    event: HistoryEvent,
    current_history_idx: usize,
) -> Element {
    let str = format!("{}", event);
    rsx! {
        div {
            min_height: "40px",
            class: "history-item".to_string() +
            match ln == current_history_idx {
                true => " selected",
                false => ""
            },
            onclick: move |_| {
                if let Some(f) = tabs.write().get_current_file_mut() { f.go_to_history_idx(ln + 1); }
            },
            "{str}"
        }
    }
}
