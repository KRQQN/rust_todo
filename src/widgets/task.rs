use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
    widgets::ListItem,
};

use chrono::{DateTime, Local};

#[derive(Clone)]
pub struct Task {
    pub text: String,
    pub description: String,
    pub done: bool,
    pub reminder: Option<DateTime<Local>>,
    pub created_at: DateTime<Local>,
}

impl Task {
    pub fn new(text: String, description: String, reminder: Option<DateTime<Local>>) -> Self {
        Self {
            text,
            description,
            done: false,
            reminder,
            created_at: Local::now(),
        }
    }

    pub fn empty() -> Self {
        Self {
            text: String::new(),
            description: String::new(),
            done: false,
            reminder: None,
            created_at: Local::now(),
        }
    }
    pub fn as_list_item(&self) -> ListItem<'_> {
        let checkbox = if self.done { "[x]" } else { "[ ]" };

        let style = if self.done {
            Style::default().fg(Color::Green)
        } else {
            Style::default()
        };

        let content = Line::from(vec![
            Span::styled(checkbox, style),
            Span::raw(" "),
            Span::raw(&self.text),
        ]);

        ListItem::new(content)
    }
}

impl Default for Task {
    fn default() -> Self {
        Self::empty()
    }
}
