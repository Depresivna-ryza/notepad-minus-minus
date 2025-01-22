use itertools::Itertools;

pub struct Text {
    content: Vec<Vec<char>>,
    caret_line: usize,
    caret_column: usize,
    old_content: Vec<Vec<char>>,
}

impl Text {
    pub fn new(content: String) -> Self {
        let content = content.lines().map(|line| line.chars().collect_vec()).collect_vec();

        Self {
            content: content.clone(),
            caret_line: 0,
            caret_column: 0,
            old_content: content.clone(),
        }
    }

    pub fn to_string(&self) -> String {
        self.content.iter().map(|line| line.iter().collect()).collect::<Vec<String>>().join("\n")
    }

    fn get_cols(&self, line: usize) -> usize {
        // self.content.get(line).map(|line| line.len()).unwrap_or(0)
        self.content.get(line).map(|line| line.len()).unwrap()
    }

    pub fn lines(&self) -> Vec<String> {
        self.content.iter().map(|line| line.iter().collect()).collect()
    }

    pub fn lines_chars(&self) -> Vec<Vec<&char>> {
        self.content.iter().map(|line| line.iter().collect()).collect()
    }

    pub fn caret_move_left(&mut self) {
        match (self.caret_line, self.caret_column) {
            (_, c) if c > 0 => self.caret_column -= 1,
            (l, _) if l > 0 => {
                self.caret_line -= 1;
                self.caret_column = self.get_cols(self.caret_line) - 1;
            }
            _ => {}
        }
    }

    pub fn caret_move_right(&mut self) {
        match (self.caret_line, self.caret_column) {
            (l, c) if c < self.get_cols(l) => self.caret_column += 1,
            (l, _) if l + 1 < self.content.len() => {
                self.caret_line += 1;
                self.caret_column = 0;
            }
            _ => {}
        }
    }

    pub fn insert_char(&mut self, c: char) {
        self.content[self.caret_line].insert(self.caret_column, c);
        self.caret_move_right();
    }

    pub fn insert_newline(&mut self) {
        let right_part: Vec<char> = self.content[self.caret_line].drain(self.caret_column..).collect();
        self.content.insert(self.caret_line + 1, right_part);

    }
 
}