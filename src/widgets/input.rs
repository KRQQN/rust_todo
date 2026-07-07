use crossterm::event::{KeyCode, KeyEvent};

pub struct Input {
    pub input: String,
    pub character_index: usize,
    pub active: bool,
    pub messages: Vec<String>,
}

impl Input {
    pub const fn new() -> Self {
        Self {
            input: String::new(),
            character_index: 0,
            messages: Vec::new(),
            active: false,
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        self.active = true;
        match key.code {
            KeyCode::Enter => self.submit_message(),
            KeyCode::Char(c) => self.enter_char(c),
            KeyCode::Backspace => self.delete_char(),
            KeyCode::Left => self.move_cursor_left(),
            KeyCode::Right => self.move_cursor_right(),
            KeyCode::Esc => self.active = false,
            _ => {}
        }
    }

    fn submit_message(&mut self) {
        if !self.input.trim().is_empty() {
            self.messages.push(self.input.trim().to_string());
        }
        self.input.clear();
        self.character_index = 0;
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.input.insert(index, new_char);
        self.move_cursor_right();
    }

    fn byte_index(&self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.input.len())
    }

    fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.character_index != 0;
        if is_not_cursor_leftmost {
            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;

            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            let after_char_to_delete = self.input.chars().skip(current_index);

            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.chars().count())
    }
}
