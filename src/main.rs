pub mod models;
pub mod views;

use std::rc::Rc;
use dioxus::desktop::window;
use models::panels::ShownPanels;
use models::tabs::Tabs;
use tracing::info;
use views::edit_history::EditHistory;
use views::editor::Editor;
use views::file_explorer::file_explorer::FileExplorer;
use views::sessionexplorer::SessionsExplorer;
use views::side_panel::SidePanel;
use views::tabs::EditorTabs;

use dioxus::prelude::*;
use views::terminal::Terminal;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    // dotenv().ok();
    launch(Layout);
}

#[component]
pub fn Layout() -> Element {
    let tabs = use_signal(Tabs::new);
    let shown_panels = ShownPanels::new();
    
    let mut terminal_height = use_signal(|| 200);
    let mut left_panel_width = use_signal(|| 100);

    let mut div_element = use_signal(|| None as Option<Rc<MountedData>>);

    let mut is_terminal_slider_pressed = use_signal(|| false);
    let mut is_left_panel_slider_pressed = use_signal(|| false);

    let handleMouseMovement = move |event: MouseEvent| async move {
        if *is_terminal_slider_pressed.read() {
            let mouse_height = event.page_coordinates().y as i32;

            let new_height = window().inner_size().height as i32 - mouse_height;
            if (new_height) < 69 {
                info!("Terminal too small");
                return;
            }
            if (new_height) > 500 {
                info!("Terminal too big");
                return;
            }
            terminal_height.set(new_height);
        }

        if *is_left_panel_slider_pressed.read() {
            let mouse_width = event.page_coordinates().x as i32;
            let new_width = mouse_width - 50;
            if (new_width) < 42 {
                info!("Left panel too small");
                return;
            }
            if (new_width) > 500 {
                info!("Left panel too big");
                return;
            }
            left_panel_width.set(new_width);
        }
    };

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        div {
            style: "display: flex; flex-direction: row; width: 100vw ; height: 100vh;",
            SidePanel {shown_panels}
            div {
                style: "display: flex; flex-direction: column; flex: 1; max-height: 100%; overflow: hidden",
                onmounted: move |el| div_element.set(Some(el.data())),
                onmousemove: handleMouseMovement,
                onmouseup: move |_| {
                    is_terminal_slider_pressed.set(false);
                    is_left_panel_slider_pressed.set(false);
                },

                div {
                    style: "display: flex; flex-direction: row; flex: 1; overflow: hidden;",
                    div {
                        style: "display: flex; flex-direction: row; overflow: hidden;",
                        display: if
                            !*shown_panels.search.read() &&
                            !*shown_panels.file_tree.read() &&
                            !*shown_panels.sessions.read() &&
                            !*shown_panels.history.read() {"none"} else {"flex"},
                        LeftPanel {
                            tabs,
                            width: left_panel_width,
                            shown_panels
                        }
                        div {
                            onmousedown: move |_| is_left_panel_slider_pressed.set(true),
                            style: "border-right: 3px solid red; cursor: ew-resize",
                        }
                    }
                    RightPanel {tabs}
                }

                div {
                    hidden: !*shown_panels.terminal.read(),
                    div {
                        onmousedown: move |_| is_terminal_slider_pressed.set(true),
                        style: "border-top: 3px solid red; cursor: ns-resize",
                    }
                    Terminal {
                        terminal_height: terminal_height
                    }
                }
            }
        }

    }
}

#[component]
pub fn LeftPanel(tabs: Signal<Tabs>, width: Signal<i32>, shown_panels: ShownPanels) -> Element {
    rsx! {
        div {
            display: "flex",
            flex_direction: "column",
            flex: 1,
            width: width.read().to_string() + "px",
            div {
                style: "display: flex; flex-direction: column; flex: 1; max-height: 100%; overflow: hidden",
                display: if !*shown_panels.file_tree.read() {"none"} else {"flex"},
                FileExplorer {tabs}
            }
            div {
                style: "display: flex; flex-direction: column; flex: 1",
                display: if !*shown_panels.sessions.read() {"none"} else {"flex"},
                SessionsExplorer {}
            }
            div {
                style: "display: flex; flex-direction: column; flex: 1; background-color: yellow",
                display: if !*shown_panels.search.read() {"none"} else {"flex"},
                input {
                    style: "",
                    type: "text",
                    placeholder: "Search",
                }
            }
            div {
                style: "display: flex; flex-direction: column; flex: 1; max-height: 100%; overflow: hidden",
                display: if !*shown_panels.history.read() {"none"} else {"flex"},
                EditHistory {tabs}
            }
        }
    }
}

#[component]
pub fn RightPanel(tabs: Signal<Tabs>) -> Element {
    rsx! {
        div {
            style: "display: flex; flex-direction: column; background-color: #ddd; flex: 1;overflow: hidden;",
            EditorTabs {tabs}
            Editor {tabs}
        }
    }
}
