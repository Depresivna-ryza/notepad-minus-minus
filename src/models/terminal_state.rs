use uuid::Uuid;


#[derive(Debug, Clone)]
pub struct TerminalData {
    pub command: String,
    pub id : Uuid,
}

#[derive(Debug, Clone)]
pub struct TerminalStates{
    pub states: Vec<TerminalData>,
    pub active_index: Option<usize>,
    pub buffers: Vec<String>,
    pub input_texts: Vec<String>,
}

impl Default for TerminalStates {
    fn default() -> Self {
        Self {
            states: vec![],
            active_index: None,
            buffers: vec![],
            input_texts: vec![],
        }
    }
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
    }

    pub fn len(&self) -> usize {
        self.states.len()
    }
}

impl TerminalData {
    pub fn new(command: String) -> Self {
        Self {
            command,
            id: Uuid::new_v4()
        }
    }
}

impl PartialEq for TerminalData {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
