use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct TerminalData {
    pub command: String,
    pub id: Uuid,
}

#[derive(Debug, Clone, Default)]
pub struct TerminalStates {
    pub states: Vec<TerminalData>,
    pub active_index: Option<usize>,
    pub buffers: Vec<String>,
    pub input_texts: Vec<String>,
}

impl TerminalStates {
    pub fn push(&mut self, terminal_data: TerminalData) {
        self.states.push(terminal_data);
        self.buffers.push("".to_string());
        self.input_texts.push("".to_string());
    }

    pub fn remove(&mut self, index: usize) {
        self.states.remove(index);
        self.buffers.remove(index);
        self.input_texts.remove(index);

        if self.active_index == Some(index) {
            self.active_index = None;
        }

        let Some(active_i) = self.active_index else {
            return;
        };

        if active_i > index {
            self.active_index = Some(active_i - 1);
        } else if active_i == index && active_i == self.len() {
            self.active_index = None;
        }
    }

    pub fn len(&self) -> usize {
        self.states.len()
    }

    pub fn is_empty(&self) -> bool {
        self.states.is_empty()
    }
}

impl TerminalData {
    pub fn new(command: String) -> Self {
        Self {
            command,
            id: Uuid::new_v4(),
        }
    }
}

impl PartialEq for TerminalData {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
