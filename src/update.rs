use chrono::Local;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{
    app::{App, InputMode},
    widgets::add_task_form::TaskFormResponse,
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
                app.add_task_form.show = true;
                app.input_mode = InputMode::Write;
            }

            KeyCode::Char('d') => {
                app.tasklist.remove(app.selected_task);
            }

            KeyCode::Char('s') => app.task_stats.sync_task_stats(&app.tasklist),

            KeyCode::Down if app.selected_task < app.tasklist.len().saturating_sub(1) => {
                app.selected_task += 1;
            }

            KeyCode::Up if app.selected_task != 0 => {
                app.selected_task -= 1;
            }

            KeyCode::Enter => {
                if let Some(task) = app.tasklist.get_mut(app.selected_task) {
                    task.done = !task.done;
                    task.completed_at = Some(Local::now());
                }
            }
            _ => {}
        },

        InputMode::Write => {
            if app.add_task_form.show {
                match app.add_task_form.handle_key(key_event) {
                    Some(TaskFormResponse::Submitted(task)) => {
                        app.tasklist.push(task);
                        app.add_task_form.show = false;
                        app.input_mode = InputMode::Menu;
                    }
                    Some(TaskFormResponse::Canceled) => {
                        app.add_task_form.show = false;
                        app.input_mode = InputMode::Menu;
                    }
                    Some(TaskFormResponse::Closed) => {
                        app.add_task_form.show = false;
                        app.input_mode = InputMode::Menu;
                    }
                    _ => {}
                }
            }
        }
    }
    app.liststate.select(Some(app.selected_task));
}
