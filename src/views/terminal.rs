use std::{cell::RefCell, env, process::{ChildStdout, Stdio}, rc::Rc, time::Duration, vec};


use dioxus::prelude::*;
use tokio::{io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader}, process::{Child, Command}, time::sleep};


async fn launch_sh(shell: String) -> Rc<RefCell<Child>> {
    Rc::new(RefCell::new(Command::new(shell)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start sh")))
}

#[component]
pub fn Terminal(hidden: Signal<bool>) -> Element {

    dbg!("Terminal hidden: {}", hidden.peek());
    
    let future = use_resource(|| async move {

        let shell = env::var("SHELL")
            .unwrap_or_else(|_| {
                dbg!("No SHELL env var found, using cmd");
                "cmd".to_string()
            });

        sleep(Duration::from_secs(3)).await;
        launch_sh(shell).await
    });

    let mut input_text = use_signal(|| "".to_string());

    rsx! {
        div {
            tabindex: 0,
            id: "terminal",
            onclick:  move |_| async  {

            },
            onfocus: move |_| {
                println!("Terminal focused");
            },
            hidden: hidden,
            style: "background-color: lightgreen; overflow-x: auto; overflow-y: auto; height: 200px;",
            "Terminal"

            TerminalText {future}

            input {
                style: "background-color: white; color: black;",
                value: input_text,
                onkeydown: move |event| {
                    let key = event.key();
                    if let Key::Enter = key {
                        let input = input_text.read().clone();
                        println!("Input: {}", input);
                        input_text.write().clear();
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
fn TerminalText(future: Resource<Rc<RefCell<Child>>>) -> Element {
    if let Some(sh1) = &*future.read_unchecked() {
        let mut buffer = use_signal(|| "PS1".to_string());
        
        let sh2 = Rc::clone(sh1);
        use_future(move ||{
            let sh = Rc::clone(&sh2);
            async move {
                let mut binding1 = sh.borrow_mut();
                let stdin = binding1.stdin.as_mut().unwrap();
                stdin.write("prompt [$P]$G".as_bytes()).await.unwrap();

                let mut buf = String::new();

                dbg!("Wrote to stdin");
                let stdout = binding1.stdout.as_mut().expect("Failed to get stdout");
                
                let mut stdout = BufReader::new(stdout);

                let res = stdout.read_line(&mut buf).await.expect("Failed to read stdout");
                dbg!("Read from stdout: {}", res);
                *buffer.write() = buf;
            } 
        });

        rsx! {
            pre {
                style: "background-color: white; color: black;",
                "{buffer}"
            }
        }
    } else {
        rsx! {
            TerminalLoading {}
        }
    }
}