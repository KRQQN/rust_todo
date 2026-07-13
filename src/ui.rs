use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
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
            Constraint::Percentage(95),
            Constraint::Length(3),
        ])
        .split(frame.area());

    match app.input_mode {
        InputMode::Menu => {
            Header::render(frame, layout[0]);

            let main_areas = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Constraint::Percentage(45), Constraint::Percentage(55)])
                .split(layout[1]);

            Tasklist::render(frame, main_areas[0], app);
            app.task_stats.render(frame, main_areas[1]);

            NavigationMenu::render_main_menu(frame, layout[2]);
        }
        InputMode::Write => {
            app.add_task_form.render(frame, layout[1]);
        }
    }
}
