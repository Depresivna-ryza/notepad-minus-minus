use dioxus::prelude::*;
use dioxus_heroicons::{mini::Shape, Icon};

use std::{process::Stdio, sync::Arc, time::Duration, vec};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    process::{Child, Command},
    sync::RwLock,
    time::{sleep, timeout},
};

use crate::models::terminal_state::{TerminalData, TerminalStates};

async fn launch_sh(shell: String) -> Result<Arc<RwLock<Child>>, std::io::Error> {
    Ok(Arc::new(RwLock::new(
        Command::new(shell)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?,
    )))
}

#[component]
fn TerminalLauncher(terminal_states: Signal<TerminalStates>) -> Element {
    let mut input_text: Signal<String> = use_signal(|| "".to_string());

    let mut launch = move || {
        let cmd = input_text.read().to_string();
        terminal_states.write().push(TerminalData::new(cmd));
        terminal_states.write().active_index = Some(terminal_states().len() - 1);
    };

    return rsx! {
        div {
            class: "terminal-launcher-input",
            input {
                style: "margin-top: 10px; padding: 10px; outline: none; border: 1px solid #555555; width: 80%; max-width: 300px; background-color: #2e2e2e; color: #ffffff;",
                oninput: move |e| *input_text.write() = e.value(),
                value: input_text,
                placeholder: "Enter command to launch terminal (cmd)",
                onkeydown: move |event| {
                    if event.key() == Key::Enter {
                        launch();
                    }
                }
            }
            button {
                class: "terminal-launcher-button",
                onclick: move |_| launch(),
                "Launch terminal"
            }
        }
    };
}

static ICON_SIZE: u32 = 20;
static ICON_STYLE: &str = "margin: 5px 0; padding: 10px; border: none; 
    border-radius: 3px; cursor: pointer; display: flex; align-items: center; 
    justify-content: center; height: 25px; width: 40px";

static HIGHLIGHT_COLOR: &str = "#6794a1";
static DEFAULT_COLOR: &str = "#282c34";

#[component]
pub fn Terminal() -> Element {
    let mut terminal_states: Signal<TerminalStates> = use_signal(TerminalStates::default);

    rsx! {
            div {
            style: "display: flex; height: 100%;",

            div {
                tabindex: 0,
                style: "background-color: black; color: white; height: 100%;  display: flex; flex-direction: row; flex: 1",
                for (index, _) in terminal_states().states.iter().enumerate() {
                    div {
                        style: "display: flex; flex: 1;",
                        display: if terminal_states().active_index == Some(index) {
                                "flex"
                            } else {
                                "none"
                            },
                        ConcreteTerminal {
                            terminal_states: terminal_states,
                            index
                        }
                    }
                }

                if terminal_states().active_index.clone().is_none() {
                    TerminalLauncher {
                        terminal_states: terminal_states
                    }
                }
            }

            div {
                style: "display: flex; flex-direction: column; background-color: rgb(15, 16, 24);
                        border-left: solid rgb(50, 52, 87) 1px; padding: 10px; overflow-y: auto;",
                for (index, terminal_state) in terminal_states().states.iter().enumerate() {
                    div {
                        style: "display: flex; position: relative;",
                        button {
                            style: ICON_STYLE,
                            title: terminal_state.command.clone(),
                            background_color: if terminal_states().active_index == Some(index) {HIGHLIGHT_COLOR} else {DEFAULT_COLOR},
                            color: "white",
                            onclick: move |_| terminal_states.write().active_index = Some(index),
                            Icon {
                                fill: if terminal_states().active_index == Some(index) {"black"} else {"rgb(185, 185, 185)"},
                                icon: Shape::CommandLine,
                                size: ICON_SIZE,
                            }
                        }

                        div {
                            style: "position: absolute; top: 0; right: -6px; color: white;
                                margin: 0;  border-radius: 50%; cursor: pointer; 
                                width: 14px; height: 14px; display: flex; justify-content: center; align-items: center;",
                            background_color: if terminal_states().active_index == Some(index ) {"rgb(97, 97, 97)"} else {"rgb(48, 48, 48)"},

                            onclick: move |_| {
                                let index = index;
                                terminal_states.write().remove(index);
                            },
                            Icon {
                                fill: if terminal_states().active_index == Some(index) {"white"} else {"grey"},
                                icon: Shape::XMark,
                            }
                        }
                    }
                }

                button {
                    style: ICON_STYLE,
                    title: "Launch terminal".to_string(),
                    background_color: if terminal_states().active_index.clone().is_none() {HIGHLIGHT_COLOR} else {DEFAULT_COLOR},
                    color: "#282c34",
                    oncontextmenu: move |_| {
                        terminal_states.write().push(TerminalData::new("cmd".to_string()));
                        terminal_states.write().active_index = Some(terminal_states().len() - 1);
                    },
                    onclick: move |_| terminal_states.write().active_index = None,
                    Icon {
                        fill: if terminal_states().active_index.clone().is_none() {"black"} else {"rgb(185, 185, 185)"},
                        icon: Shape::Plus,
                        size: ICON_SIZE,
                    }
                }
            }
        }
    }
}

static TERMINAL_STYLE: &str = "background-color: rgb(6, 7, 17); color: white; 
    width: 100% ; flex: 1; overflow-y: scroll;
    margin: 0; padding: 0; border: 0; cursor: text; scroll-behavior: smooth";

#[component]
fn TerminalLoading() -> Element {
    rsx! {
        pre {
            style: TERMINAL_STYLE,
            "Loading..."
        }
    }
}

#[component]
fn ConcreteTerminal(terminal_states: Signal<TerminalStates>, index: usize) -> Element {
    let command2 = terminal_states().states[index].command.clone();
    let process = use_resource(move || {
        let command = command2.clone();
        async move {
            sleep(Duration::from_secs(1)).await;
            launch_sh(command).await
        }
    });

    let Some(ref sh1) = *process.read_unchecked() else {
        return rsx! {
            TerminalLoading {}
        };
    };

    let Ok(sh1) = sh1 else {
        return rsx! {
            div {
                onmounted: move |_| async move { sleep(Duration::from_secs(5)).await; terminal_states.write().remove(index); },
                class: "terminal-launcher-input",
                div {
                    style: "padding: 10px; background-color: rgba(255, 185, 185, 0.61); color: black; font-size: 15px; font-weight: 500;
                        text-align: center; font-family: 'JetBrains Mono';",
                    "Failed to launch terminal with command: \"{terminal_states().states[index].command.clone()}\""
                }
            }
        };
    };

    let mut commands: Signal<String> = use_signal(|| "".to_string());

    let write_rc = Arc::clone(sh1);
    let _ = use_resource(move || {
        //pushing commands to stdin
        let sh = Arc::clone(&write_rc);
        async move {
            let mut sh = sh.write().await;
            let stdin = sh.stdin.as_mut().unwrap();
            if commands.read_unchecked().is_empty() {
                return;
            }
            stdin
                .write_all(commands.read_unchecked().as_bytes())
                .await
                .expect("Failed to write to stdin");
            stdin.flush().await.expect("Failed to flush stdin");
            dbg!(commands.read());
        }
    });

    let read_rc = Arc::clone(sh1);
    let _ = use_future(move || {
        //pulling commands from stdout
        let sh = Arc::clone(&read_rc);
        async move {
            loop {
                sleep(Duration::from_millis(1)).await;
                let mut sh = sh.write().await;
                let stdout = sh.stdout.as_mut().expect("Failed to get stdout");
                let mut outBuf = BufReader::new(stdout);

                let Ok(buf) = timeout(Duration::from_millis(200), outBuf.fill_buf()).await else {
                    continue;
                };

                let Ok(buf) = buf else {
                    dbg!("Failed to read from stdout");
                    continue;
                };

                let buffer_str = &*String::from_utf8_lossy(buf);
                terminal_states.write().buffers[index].push_str(buffer_str);
            }
        }
    });

    rsx! {
        div {
            style: "display: flex; flex-direction: column; height: 100%; width: 100%; ",
            pre {
                style: TERMINAL_STYLE,
                background_color: "rgb(6, 7, 17)",
                color: "rgb(140, 255, 111)",
                "{terminal_states().buffers[index]}"
            }
            input {
                style: "color: white; height: 30px; width: 100%; margin: 0; padding: 0; border: 0;",
                background_color: "rgb(19, 18, 34)",
                value: terminal_states().input_texts[index].clone(),

                oninput: move |event| {
                    let val = event.value();
                    terminal_states.write().input_texts[index] = val.clone();
                },

                onkeydown: move |event| {
                    if event.key() == Key::Enter {
                        *commands.write() = format!("{}\n", terminal_states().input_texts[index]);
                        terminal_states.write().input_texts[index] = "".to_string();
                    }
                }
            }
        }
    }
}
