use crate::app::App;
use ratatui::layout::Rect;
use ratatui::prelude::*;
use ratatui::widgets::{Block, BorderType, Borders, List, ListItem, Padding};

pub struct Tasklist;

impl Tasklist {
    pub fn render(frame: &mut Frame, area: Rect, app: &mut App) {
        let tasks: Vec<ListItem> = app
            .tasklist
            .iter()
            .map(|task| -> ListItem<'_> { task.as_list_item() })
            .collect();

        let list = List::new(tasks)
            .block(
                Block::default()
                    .title("| tasks |")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Thick)
                    .border_style(Style::default().fg(Color::Yellow))
                    .style(Style::default().bg(Color::Black))
                    .padding(Padding::new(1, 1, 1, 1)),
            )
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol(">>");

        frame.render_stateful_widget(list, area, &mut app.liststate);
    }
}
