use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph},
};

pub struct NavigationMenu {}

impl NavigationMenu {
    pub fn render(frame: &mut Frame, area: Rect) {
        frame.render_widget(
            Paragraph::new(Line::from(vec![
                " [A] ".into(),
                " add task ".into(),
                " [↑↓] ".into(),
                " navigate ".into(),
                " [Enter] ".into(),
                " select ".into(),
                "[Q]".into(),
                " quit ".into(),
            ]))
            .block(
                Block::default()
                    .title("")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center),
            area,
        );
    }
}
