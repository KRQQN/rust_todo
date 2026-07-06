use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::app::App;

pub fn render(app: &mut App, frame: &mut Frame) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(5),
            Constraint::Percentage(85),
            Constraint::Percentage(10),
        ])
        .split(frame.area());

    frame.render_widget(
        Paragraph::new("")
            .block(
                Block::default()
                    .title("Task Manager")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::TOP)
                    .border_type(BorderType::Thick),
            )
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center),
        layout[0],
    );
    frame.render_widget(
        Paragraph::new("")
            .block(
                Block::default()
                    .title("Tasks")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center),
        layout[1],
    );
    frame.render_widget(
        Paragraph::new(Line::from(vec![
            "[Q]".into(),
            " quit ".into(),
            " [↑↓] ".into(),
            " navigate ".into(),
            " [Enter] ".into(),
            " select ".into(),
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
        layout[2],
    );
}
