use std::{cell::RefCell, env, io::stdout, ops::DerefMut, process::{ChildStdout, Stdio}, rc::Rc, sync::{mpsc::{channel, Receiver}, Arc}, time::Duration, vec};


use dioxus::prelude::*;
use dioxus_elements::div;
use svg_attributes::d;
use tokio::{io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader}, process::{Child, Command}, sync::{ RwLock}, time::{sleep, timeout}};
use tracing::info;


async fn launch_sh(shell: String) -> Arc<RwLock<Child>> {
    Arc::new(RwLock::new(Command::new(shell)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start sh")))
}

#[component]
pub fn Terminal(hidden: Signal<bool>) -> Element {

    dbg!(hidden.peek());
    
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
            hidden: hidden,
            style: "height: 200px",
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
        let (tx, mut rx) = channel::<String>();

        let write_rc = Arc::clone(sh1); //idk how to to it nicely :/
        //pushing commands to stdin
        tokio::spawn(async move {
                loop {
                    sleep(Duration::from_millis(10)).await;
                    if let Ok(val) = rx.recv() {
                        let mut sh = write_rc.write().await;
                        // let mut binding =  sh.deref_mut();
                        let stdin = sh.stdin.as_mut().unwrap();
                        let res = stdin.write_all(val.as_bytes()).await.expect("Failed to write to stdin");
                        // dbg!(commands.peek());
                        dbg!(res);
                        dbg!(val);
                    }
                }
        });

        let read_rc = Arc::clone(sh1); //idk how to to it nicely :/
        let _ = use_resource(move || { //pulling commands from stdout
            let sh = Arc::clone(&read_rc); //idk how to to it nicely :/
            async move {
                loop {
                    sleep(Duration::from_millis(10000)).await;
                    let mut sh = sh.write().await;
                    let stdout = sh.stdout.as_mut().expect("Failed to get stdout");
                    dbg!("Reading from stdout");
                    let mut outBuf = BufReader::new(stdout);
                    let Ok(buf) =  timeout(Duration::from_millis(200), outBuf.fill_buf()).await else {
                        dbg!("Failed to read from stdout1");
                        continue;
                    };
                    let Ok(buf) = buf else {
                        dbg!("Failed to read from stdout2");
                        continue;
                    };

                    let buffer_str = &*String::from_utf8_lossy(buf);
                    buffer.write().push_str(buffer_str);
                    dbg!(buffer_str);
                }
            }
        });

        rsx! {
            div {  
                style: "display: flex; flex-direction: column; height: 100%;",
                textarea {
                    style: "background-color: black; color: white; width: 100%; flex: 1; overflow: scroll; margin: 0; padding: 0; border: 0;",
                    readonly: true,
                    value: buffer,
                }
            
            input {
                style: "background-color: black; color: white; height: 30px; width: 100%; margin: 0; padding: 0; border: 0;",
                value: input_text,
                
                oninput: move |event| {
                    let val = event.value();
                    dbg!("Input: {}", val.clone());
                    *input_text.write() = val.clone();
                    // *commands.write() = input_text.read().to_string();
                    
                },
                
                onkeydown: move |event| {
                    if tx.send(event.key().to_string()).is_err() {
                        dbg!("Failed to send to channel");
                    }
                    if event.key() == Key::Enter {
                        dbg!("Enter pressed");
                        *input_text.write() = "".to_string();
                        *commands.write() = "\n".to_string();
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