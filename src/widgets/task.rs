use chrono::{DateTime, Local};
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
    widgets::ListItem,
};
use serde::{Deserialize, Serialize};

use crate::utils::time_formatter::ReminderTimeFormatter;

#[derive(Clone, Serialize, Deserialize)]
pub struct Task {
    pub text: String,
    pub description: String,
    pub done: bool,
    pub reminder: Option<DateTime<Local>>,
    pub completed_at: Option<DateTime<Local>>,
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
            completed_at: None,
        }
    }

    pub fn empty() -> Self {
        Self {
            text: String::new(),
            description: String::new(),
            done: false,
            reminder: None,
            created_at: Local::now(),
            completed_at: None,
        }
    }

    pub fn as_list_item(&self) -> ListItem<'_> {
        let checkbox = if self.done { "[x]" } else { "[ ]" };

        let mut spans = vec![
            Span::styled(
                checkbox,
                Style::default().fg(if self.done {
                    Color::Green
                } else {
                    Color::White
                }),
            ),
            Span::raw(" "),
            Span::raw(&self.text),
        ];

        if let Some(span) =
            ReminderTimeFormatter::format_reminder(self.reminder, self.completed_at, self.done)
        {
            spans.push(Span::raw(" "));
            spans.push(span);
        }

        ListItem::new(Line::from(spans))
    }
}

impl Default for Task {
    fn default() -> Self {
        Self::empty()
    }
}
