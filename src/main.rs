pub mod models;
pub mod views;

use dioxus::desktop::window;
use models::file_system::FileSystem;
use models::panels::ShownPanels;
use models::tabs::Tabs;
use std::rc::Rc;
use tracing::info;
use views::dialogs::fs_operations::OperationDialogHandler;
use views::editor::Editor;
use views::file_explorer::context_menu::{RightClickMenu, RightClickMenuHandler};
use views::file_explorer::file_explorer::FileExplorer;
use views::sessionexplorer::SessionsExplorer;
use views::side_panel::SidePanel;
use views::tabs::EditorTabs;
use views::{edit_history::EditHistory, find_replace::FindReplace};

use crate::views::dialogs::error::{ErrorDialog, ErrorDialogHandler};

use dioxus::prelude::*;
use views::terminal::Terminal;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const BOOTSTRAP_CSS: Asset = asset!("/assets/css/bootstrap.min.css");

fn main() {
    // dotenv().ok();
    launch(Layout);
}

#[component]
pub fn Layout() -> Element {
    let error_dialog_handler = use_context_provider(ErrorDialogHandler::new);
    let right_click_menu_handler =
        use_context_provider(|| Signal::new(RightClickMenuHandler::new()));
    let _ = use_context_provider(|| Signal::new(FileSystem::new()));
    let _ = use_context_provider(OperationDialogHandler::new);

    let tabs = use_signal(Tabs::new);

    let shown_panels = ShownPanels::new();

    let mut terminal_height = use_signal(|| 200);
    let mut left_panel_width = use_signal(|| 210);

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
            if (new_width) < 210 {
                // info!("Left panel too small");
                return;
            }
            if (new_width) > 500 {
                // info!("Left panel too big");
                return;
            }
            left_panel_width.set(new_width);
        }
    };

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: BOOTSTRAP_CSS }

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
                            class: "left-panel-slider",
                        }
                    }
                    RightPanel {tabs}
                }

                div {
                    height: terminal_height.read().to_string() + "px",
                    hidden: !*shown_panels.terminal.read(),
                    div {
                        onmousedown: move |_| is_terminal_slider_pressed.set(true),
                        class: "terminal-slider",
                    }
                    Terminal {}
                }
            }
        }

        if right_click_menu_handler.read().is_open() {
            if let Some(fs_item) = right_click_menu_handler.read().get_fs_item() {
                RightClickMenu { fs_item }
            }
        }

        if error_dialog_handler.is_shown() {
            ErrorDialog {}
        }
    }
}
#[component]
pub fn Divider() -> Element {
    rsx! {
        div {
            style: "display: flex; min-height: 3px; background-color: rgb(139, 139, 139); width: 100%;",
        }
    }
}

#[component]
pub fn LeftPanel(tabs: Signal<Tabs>, width: Signal<i32>, shown_panels: ShownPanels) -> Element {
    rsx! {
        div {
            class: "invisible-scrollbar left-panel",
            style: "overflow-y: auto; overflow-x: hidden; display: flex; flex-direction: column; width: {width.read()}px;",
            div {
                style: "display: flex; flex-direction: column; flex: 1; overflow: hidden; min-height: 250px;",
                display: if !*shown_panels.file_tree.read() {"none"} else {"flex"},
                FileExplorer {tabs}
                Divider {}
            }
            div {
                style: "display: flex; flex: 1; min-height: 250px; width: 100%; flex-direction: column;",
                display: if !*shown_panels.sessions.read() {"none"} else {"flex"},
                SessionsExplorer {}
                Divider {}
            }
            div {
                style: "display: flex; flex-direction: column; flex: 1; background-color: yellow;",
                display: if !*shown_panels.search.read() {"none"} else {"flex"},
                FindReplace { tabs }
                Divider {}
            }
            div {
                style: "display: flex; flex-direction: column; flex: 1; max-height: 100%; overflow: hidden; min-height: 250px;",
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
