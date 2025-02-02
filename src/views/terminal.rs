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
pub fn Terminal() -> Element {
    let mut input_text: Signal<String> = use_signal(|| "".to_string());

    let mut terminal_states: Signal<Vec<String>> = use_signal(|| vec![]);
    let mut active_index: Signal<Option<usize>> = use_signal(|| Option::None);

    rsx! {
        div {
        style: "display: flex; height: 100%;",

        div {
            tabindex: 0,
            style: "background-color: black; color: white; height: 100%;  display: flex; flex-direction: row; flex: 1",
            for (index, command) in terminal_states.read().iter().enumerate() {
                div {
                    style: "display: flex; flex: 1;",
                    display: if active_index.read().clone() == Some(index) {
                            "flex"
                        } else {
                            "none"
                        },

                    ConcreteTerminal {
                        command: command.clone()
                    }
                }
            }

            if active_index.read().clone().is_none() {
                div {
                    style: "display: flex; flex-direction: column; justify-content: center; align-items: center; height: 100%; width: 100%;",
                    input {
                        style: "margin-top: 10px; padding: 10px; border: 1px solid #ccc; border-radius: 5px; width: 80%; max-width: 300px;",
                        oninput: move |e| *input_text.write() = e.value(),
                        value: input_text,
                        placeholder: "Enter command to launch terminal (cmd)",
                        onkeydown: move |event| {
                            if event.key() == Key::Enter {
                                let cmd = input_text.read().to_string();
                                terminal_states.write().push(cmd);
                                *active_index.write() = Some(terminal_states.peek().len() - 1);
                            }
                        }
                    }
                    button {
                    style: "margin-top: 10px; padding: 10px 20px; border: none; border-radius: 5px; cursor: pointer;",
                    background_color: HIGHLIGHT_COLOR,
                    color: DEFAULT_COLOR,
                    onclick: move |_| {
                            let cmd = input_text.read().to_string();
                            terminal_states.write().push(cmd);
                            *active_index.write() = Some(terminal_states.peek().len() - 1);
                        },
                        "Launch terminal"
                    }
                }
                }
            }

            div {
                style: "display: flex; flex-direction: column; background-color: rgb(15, 16, 24); border-left: solid rgb(50, 52, 87) 1px; padding: 10px; overflow-y: auto;",
                for (index, command) in terminal_states.read().iter().enumerate() {
                    div {  
                        style: "display: flex; position: relative;",
                        button {
                            style: ICON_STYLE,
                            title: command.clone(),
                            background_color: if active_index.read().clone() == Some(index) {HIGHLIGHT_COLOR} else {DEFAULT_COLOR},
                            color: "white",
                            onclick: move |_| *active_index.write() = Some(index),
                            Icon {
                                icon: Shape::CommandLine,
                                size: ICON_SIZE,
                            }
                        }
                        
                    }
                }

                button {
                    style: ICON_STYLE,
                    title: "Launch terminal".to_string(),
                    background_color: if active_index.read().clone().is_none() {HIGHLIGHT_COLOR} else {DEFAULT_COLOR},
                    color: "#282c34",
                    oncontextmenu: move |_| {
                        let len = terminal_states.read().len();
                        terminal_states.write().push("cmd".to_string()); 
                        *active_index.write() = Some(terminal_states.read().len() - 1);
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


static TERMINAL_STYLE: &str = 
    "background-color: rgb(6, 7, 17); color: white; 
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
fn ConcreteTerminal(command: String) -> Element {
    let process = use_resource(move ||{
        let command = command.clone();
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

    let mut buffer = use_signal(|| "".to_string());
    let mut input_text = use_signal(|| "".to_string());
    let mut commands = use_signal(|| "".to_string());

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
    use_future(move || {
        //pulling commands from stdout
        let sh = Arc::clone(&read_rc); 
        async move {
            loop {
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
                style: TERMINAL_STYLE,
                background_color: "rgb(6, 7, 17)",
                color: "rgb(140, 255, 111)",
                "{buffer.read()}"
            }

            input {
                style: "color: white; height: 30px; width: 100%; margin: 0; padding: 0; border: 0;",
                background_color: "rgb(19, 18, 34)",
                value: input_text,

                oninput: move |event| {
                    let val = event.value();
                    *input_text.write() = val.clone();
                },

                onkeydown: move |event| {
                    if event.key() == Key::Enter {
                        *commands.write() = format!("{}\n", input_text.read());
                        *input_text.write() = "".to_string();
                    }
                }
            }
        }
    }
}
