use dioxus::prelude::*;
use dioxus_heroicons::{mini::Shape, Icon};
use std::{
    process::Stdio,
    sync::Arc,
    time::Duration,
    vec,
};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    process::{Child, Command},
    sync::RwLock,
    time::{sleep, timeout},
};


async fn launch_sh(shell: String) -> Arc<RwLock<Child>> {
    Arc::new(RwLock::new(Command::new(shell)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start sh")))
}


static ICON_SIZE: u32 = 20;
static ICON_STYLE: &str = 
    "margin: 5px 0; padding: 10px; border: none; 
    border-radius: 3px; cursor: pointer; display: flex; align-items: center; 
    justify-content: center; height: 25px; width: 40px";

static HIGHLIGHT_COLOR: &str = "#61dafb";
static DEFAULT_COLOR: &str = "#282c34";


#[component]
fn TerminalLauncher(terminal_states: Signal<Vec<String>>, active_index: Signal<Option<i32>>) -> Element {
    let mut input_text: Signal<String> = use_signal(|| "".to_string());

    return rsx! {
        div {
            style: "display: flex; flex-direction: column; justify-content: center; align-items: center; height: 100%; width: 100%;",
            input {
                style: "margin-top: 10px; padding: 10px; border: 1px solid #ccc; border-radius: 5px; width: 80%; max-width: 300px;",
                oninput: move |e| *input_text.write() = e.value(),
                value: input_text,
                placeholder: "Enter command to launch terminal (cmd)",
                onkeydown: move |event| {
                    if event.key() == Key::Enter {
                        terminal_states.write().push(input_text.read().to_string());
                        *active_index.write() = Some(terminal_states.read().len() as i32 - 1);
                    }
                }
            }
            button {
            style: "margin-top: 10px; padding: 10px 20px; border: none; border-radius: 5px; background-color: #61dafb; color: #282c34; cursor: pointer;",
            onclick: move |_| {
                    terminal_states.write().push(input_text.read().to_string());
                    *active_index.write() = Some(terminal_states.read().len() as i32 - 1);
                },
                "Launch terminal"
            }
        }
    }
}

#[component]
pub fn Terminal() -> Element {
    let mut terminal_states: Signal<Vec<String>> = use_signal(|| vec![]);
    let mut active_index: Signal<Option<i32>> = use_signal(|| Option::None);

    rsx! {
        div {
        style: "display: flex; height: 100%;",

        div {
            tabindex: 0,
            style: "background-color: black; color: white; height: 100%; width: 100%;  display: flex; flex-direction: row; flex: 1",
            for (index, command) in terminal_states.read().iter().enumerate() {
                div {
                    style: "display: flex; flex: 1; width: 100%;",
                    display: if active_index.read().clone() == Some(index as i32) {
                            "flex"
                        } else {
                            "none"
                        },
                    ConcreteTerminal {
                        command: command.clone(),
                    }
                }
            }

            if active_index.read().clone().is_none() {
                    TerminalLauncher {
                        terminal_states: terminal_states,
                        active_index: active_index,
                    }
                }
            }

            div {
                style: "display: flex; flex-direction: column; background-color: rgb(6, 7, 10); padding: 10px; overflow-y: auto;",
                for (index, command) in terminal_states.read().iter().enumerate() {
                    button {
                        style: ICON_STYLE,
                        title: command.clone(),
                        background_color: if active_index.read().clone() == Some(index as i32) {HIGHLIGHT_COLOR} else {DEFAULT_COLOR},
                        color: "white",

                        onclick: move |_| *active_index.write() = Some(index as i32),
                        Icon {
                            icon: Shape::CommandLine,
                            size: ICON_SIZE,
                        }
                    }
                }

                button {
                    style: ICON_STYLE,
                    title: "Launch terminal".to_string(),
                    background_color: if active_index.read().clone().is_none() {HIGHLIGHT_COLOR} else {DEFAULT_COLOR},
                    color: "#282c34",
                    oncontextmenu: move |_| {
                        terminal_states.write().push("cmd".to_string()); 
                        *active_index.write() = Some(terminal_states.read().len() as i32 - 1);
                    },
                    onclick: move |_| *active_index.write() = None,
                    Icon {
                        icon: Shape::Plus,
                        size: ICON_SIZE,
                    }
                }
            }
        }
    }
}

#[component]
fn TerminalLoading() -> Element {
    rsx! {
        pre {
            style: "background-color: white; color: black;",
            "Loading..."
        }
    }
}

#[component]
fn ConcreteTerminal(command: String) -> Element {
    let process = use_resource(move ||{
        let command = command.clone();
        async move {
            sleep(Duration::from_secs(3)).await;
            launch_sh(command).await
        }
    });

    let Some(ref sh1) = *process.read_unchecked() else {
        return rsx! {
            TerminalLoading {}
        };
    };

    let mut buffer = use_signal(|| "".to_string());
    let mut input_text = use_signal(|| "".to_string());
    let mut commands = use_signal(|| "".to_string());

    let write_rc = Arc::clone(sh1); //idk how to to it nicely :/
    let _ = use_resource(move || {
        //pushing commands to stdin
        let sh = Arc::clone(&write_rc); //idk how to to it nicely :/
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

    let read_rc = Arc::clone(sh1); //idk how to to it nicely :/
    use_future(move || {
        //pulling commands from stdout
        let sh = Arc::clone(&read_rc); //idk how to to it nicely :/
        async move {
            loop {
                // info!("Reading from stdout");
                sleep(Duration::from_millis(10)).await;
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
                buffer.write().push_str(buffer_str);
            }
        }
    });

    rsx! {
        div {
            style: "display: flex; flex-direction: column; height: 100%; width: 100%; ",
            pre {
                style: "background-color: black; color: white; width: 100% ; flex: 1; overflow-y: scroll;
                        margin: 0; padding: 0; border: 0; cursor: text; scroll-behavior: smooth;",
                "{buffer}"
            }

            input {
                style: "background-color: black; color: white; height: 30px; width: 100%; margin: 0; padding: 0; border: 0;",
                value: input_text,

                oninput: move |event| {
                    let val = event.value();
                    *input_text.write() = val.clone();
                },

                onkeydown: move |event| {
                    if event.key() == Key::Enter {
                        dbg!("Enter pressed");
                        *commands.write() = format!("{}\n", input_text.read());
                        *input_text.write() = "".to_string();
                    }
                }
            }
        }
    }
}
