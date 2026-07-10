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

        if let Some(reminder) = self.reminder {
            let now = Local::now();
            let duration = reminder.signed_duration_since(now);

            let reminder_text = if self.done {
                "".to_string()
            } else if duration.num_days() > 0 {
                format!(" (in {}d)", duration.num_days())
            } else if duration.num_hours() > 0 {
                format!(" (in {}h)", duration.num_hours())
            } else if duration.num_minutes() > 0 {
                format!(" (in {}m)", duration.num_minutes())
            } else {
                " (now!)".to_string()
            };

            let reminder_style = if !self.done && duration.num_hours() <= 24 {
                Style::default().fg(Color::Red)
            } else if !self.done && duration.num_days() <= 7 {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default().fg(Color::LightCyan)
            };

            spans.push(Span::raw(" "));
            spans.push(Span::styled(reminder_text, reminder_style));
        }

        ListItem::new(Line::from(spans))
    }
}

impl Default for Task {
    fn default() -> Self {
        Self::empty()
    }
}
