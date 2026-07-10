use ratatui::widgets::ListState;

use crate::widgets::{add_task_form::AddTaskForm, io::Io, task::Task};

pub struct App {
    pub title: String,
    pub quit: bool,
    pub input_mode: InputMode,
    pub io: Io,
    pub tasklist: Vec<Task>,
    pub selected_task: usize,
    pub liststate: ListState,
    pub add_task_form: AddTaskForm,
}

pub enum InputMode {
    Menu,
    Write,
}

impl App {
    pub fn new() -> App {
        App {
            title: String::from("Task Manager"),
            quit: false,
            input_mode: InputMode::Menu,
            io: Io::new(),
            liststate: ListState::default(),
            add_task_form: AddTaskForm::new(),
            selected_task: 0,
            tasklist: vec![
                Task::new(
                    "Learn more rust".to_string(),
                    "...describing".to_string(),
                    None,
                ),
                Task::new(
                    "Learn ratatui".to_string(),
                    "...describing".to_string(),
                    None,
                ),
                Task::new(
                    "Create a mini todo app".to_string(),
                    "...describing".to_string(),
                    None,
                ),
            ],
        }
    }

    pub fn quit(&mut self) {
        self.quit = true;
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
