use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::{
    app::{App, InputMode},
    widgets::{header::Header, nav_footer::NavigationMenu, tasklist::Tasklist},
};

pub fn render(app: &mut App, frame: &mut Frame) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(5),
            Constraint::Percentage(85),
            Constraint::Percentage(10),
        ])
        .split(frame.area());

    match app.input_mode {
        InputMode::Menu => {
            Header::render(frame, layout[0]);
            Tasklist::render(frame, layout[1], app);
            NavigationMenu::render(frame, layout[2]);
        }

        InputMode::Write => app.io.render(frame, layout[1]),
    }
}
