use std::{env, ops::{Deref, Not}, process::Stdio, rc::Rc, sync::Arc, time::Duration, vec};
use dioxus::{desktop::{tao::event, window}, html::{canvas::height, g::direction, geometry::euclid::Rect, mo}, prelude::*};
use tokio::{io::{AsyncBufReadExt, AsyncWriteExt, BufReader}, process::{Child, Command}, sync::RwLock, time::{sleep, timeout}};
use tracing::info;

use crate::views::terminal;


async fn launch_sh(shell: String) -> Arc<RwLock<Child>> {
    Arc::new(RwLock::new(Command::new(shell)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start sh")))
}

#[component]
pub fn Terminal(terminal_height: Signal<i32>) -> Element {
    let future = use_resource(|| async move {
        let shell = env::var("SHELL")
            .unwrap_or_else(|_| {
                dbg!("No SHELL env var found, using cmd");
                "cmd".to_string()
            });

        sleep(Duration::from_secs(3)).await;
        launch_sh(shell).await
    });

    rsx! {
        div {
            tabindex: 0,
            height: terminal_height.read().to_string() + "px",
            style: "background-color: black; color: white",
            TerminalText {future}
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
fn TerminalText(future: Resource<Arc<RwLock<Child>>>) -> Element {
    if let Some(ref sh1) = *future.read_unchecked() {
        let mut buffer = use_signal(|| "".to_string());
        let mut input_text = use_signal(|| "".to_string());
        let mut commands = use_signal(|| "".to_string());

        let write_rc = Arc::clone(sh1); //idk how to to it nicely :/
        let _ = use_resource(move ||{ //pushing commands to stdin
            let sh = Arc::clone(&write_rc); //idk how to to it nicely :/
            async move {
                let mut sh = sh.write().await;
                let stdin = sh.stdin.as_mut().unwrap();
                if commands.read_unchecked().is_empty() {
                    return;
                }
                stdin.write_all(commands.read_unchecked().as_bytes())
                    .await.expect("Failed to write to stdin");
                stdin.flush().await.expect("Failed to flush stdin");
                dbg!(commands.read());
            }
        });

        let read_rc = Arc::clone(sh1); //idk how to to it nicely :/
        use_future(move || { //pulling commands from stdout
            let sh = Arc::clone(&read_rc); //idk how to to it nicely :/
            async move {
                loop {
                    sleep(Duration::from_millis(10)).await;
                    let mut sh = sh.write().await;
                    let stdout = sh.stdout.as_mut().expect("Failed to get stdout");
                    let mut outBuf = BufReader::new(stdout);

                    let Ok( buf) =  timeout(Duration::from_millis(200), outBuf.fill_buf()).await else {
                        continue;
                    };

                    let Ok(buf ) = buf else {
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
                style: "display: flex; flex-direction: column; height: 100%;",
                pre {
                    style: "background-color: black; color: white; width: 100%; flex: 1; overflow-y: scroll; 
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
    } else {
        rsx! {
            TerminalLoading {}
        }
    }
}