use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
    widgets::ListItem,
};

pub struct Task {
    pub text: String,
    pub done: bool,
}

impl Task {
    pub fn new() -> Task {
        Task {
            text: String::from(""),
            done: false,
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
