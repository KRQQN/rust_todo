use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};

pub struct Header {}

impl Header {
    pub fn render(frame: &mut Frame, area: Rect) {
        frame.render_widget(
            Paragraph::new("")
                .block(
                    Block::default()
                        .title("Task Manager")
                        .title_alignment(Alignment::Center)
                        .borders(Borders::TOP)
                        .border_type(BorderType::Plain),
                )
                .style(Style::default().fg(Color::Yellow))
                .alignment(Alignment::Center),
            area,
        );
    }
}
