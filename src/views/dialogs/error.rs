use dioxus::prelude::*;

#[derive(Clone)]
pub struct ErrorDialogHandler {
    show: Signal<bool>,
    message: Signal<Option<String>>,
}

impl Default for ErrorDialogHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl ErrorDialogHandler {
    pub fn new() -> Self {
        Self {
            show: Signal::new(false),
            message: Signal::new(None),
        }
    }

    pub fn show(&mut self, message: String) {
        self.message.set(Some(message));
        self.show.set(true);
    }

    pub fn get_message(&self) -> String {
        self.message.read().clone().unwrap_or_default()
    }

    pub fn is_shown(&self) -> bool {
        *self.show.read()
    }

    pub fn hide(&mut self) {
        self.show.set(false);
    }
}

#[component]
pub fn ErrorDialog() -> Element {
    let error_dialog_handler = use_context::<ErrorDialogHandler>();

    let close = {
        let mut error_dialog_handler = error_dialog_handler.clone();
        move |_| {
            error_dialog_handler.hide();
        }
    };

    rsx! {
        div {
            class: "dialog-overlay",
            div {
                class: "dialog",
                div {
                    class: "dialog-content",
                    p { "An error occurred." }
                    p { { error_dialog_handler.get_message() } }
                    div {
                        class: "dialog-button",
                        onclick: close,
                        "Close"
                    }
                }
            }
        }
    }
}
