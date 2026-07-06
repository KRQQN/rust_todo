use crate::app::App;
use ratatui::layout::Rect;
use ratatui::prelude::*;
use ratatui::widgets::{Block, BorderType, Borders, List, ListItem};

pub struct Tasklist;

pub struct Task {
    pub text: String,
    pub done: bool,
}

impl Tasklist {
    pub fn render(frame: &mut Frame, area: Rect, app: &mut App) {
        let tasks: Vec<ListItem> = app
            .tasklist
            .iter()
            .map(|task| -> ListItem<'_> {
                let checkbox = if task.done { "[x]" } else { "[ ]" };
                let style = if task.done {
                    Style::default().fg(Color::Green)
                } else {
                    Style::default()
                };
                ListItem::new(Line::from(vec![
                    Span::styled(checkbox, style),
                    Span::raw(" "),
                    Span::raw(&task.text),
                ]))
            })
            .collect();

        let list = List::new(tasks)
            .block(
                Block::default()
                    .title("tasks")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Thick)
                    .border_style(Style::default().fg(Color::Yellow)),
            )
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol(">>");

        frame.render_stateful_widget(
            list,
            area.centered_horizontally(Constraint::Length(50)),
            &mut app.liststate,
        );
    }
}
