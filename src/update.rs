use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::{App, InputMode};

pub fn update(app: &mut App, key_event: KeyEvent) {
    match app.input_mode {
        InputMode::Menu => match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => app.quit(),
            KeyCode::Char('c') | KeyCode::Char('C')
                if key_event.modifiers == KeyModifiers::CONTROL =>
            {
                app.quit()
            }

            KeyCode::Char('a') => {
                app.input_mode = InputMode::Write;
            }

            KeyCode::Down if app.highlighted < app.tasklist.len().saturating_sub(1) => {
                app.highlighted += 1;
            }
            KeyCode::Up if app.highlighted != 0 => {
                app.highlighted -= 1;
            }

            KeyCode::Enter => {
                if let Some(task) = app.tasklist.get_mut(app.highlighted) {
                    task.done = !task.done;
                }
            }
            _ => {}
        },

        InputMode::Write => {
            app.input.handle_key(key_event);

            if !app.input.active {
                app.input_mode = InputMode::Menu;
            }
        }
    }
    app.liststate.select(Some(app.highlighted));
}
