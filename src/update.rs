use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{
    app::{App, InputMode},
    widgets::{io::UserInputEvent, task::Task},
};

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

            KeyCode::Char('d') => {
                app.tasklist.remove(app.selected_task);
            }

            KeyCode::Down if app.selected_task < app.tasklist.len().saturating_sub(1) => {
                app.selected_task += 1;
            }

            KeyCode::Up if app.selected_task != 0 => {
                app.selected_task -= 1;
            }

            KeyCode::Enter => {
                if let Some(task) = app.tasklist.get_mut(app.selected_task) {
                    task.done = !task.done;
                }
            }
            _ => {}
        },

        InputMode::Write => match app.io.handle_key(key_event) {
            UserInputEvent::Submit(text) => {
                app.tasklist.push(Task { text, done: false });
                app.selected_task = app.tasklist.len().saturating_sub(1);
                app.input_mode = InputMode::Menu;
            }
            UserInputEvent::Cancel => {
                app.input_mode = InputMode::Menu;
            }
            UserInputEvent::None => {}
        },
    }
    app.liststate.select(Some(app.selected_task));
}
