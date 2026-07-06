use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::App;

pub fn update(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') => app.quit(),
        KeyCode::Char('c') | KeyCode::Char('C') if key_event.modifiers == KeyModifiers::CONTROL => {
            app.quit()
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
    };
    app.liststate.select(Some(app.highlighted));
}
