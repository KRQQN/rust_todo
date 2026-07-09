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
                app.add_task_form.show = true;
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

        InputMode::Write => {
            if app.add_task_form.show {
                if let Some(submitted) = app.add_task_form.handle_key(key_event) {
                    if submitted == "submitted" {
                        app.input_mode = InputMode::Menu;
                    }
                }
                if !app.add_task_form.show {
                    app.input_mode = InputMode::Menu;
                }
            }
        }
    }
    app.liststate.select(Some(app.selected_task));
}
