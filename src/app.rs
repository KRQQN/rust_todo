use ratatui::widgets::ListState;

use crate::widgets::tasklist::Task;

pub struct App {
    pub title: String,
    pub quit: bool,
    pub tasklist: Vec<Task>,
    pub highlighted: usize,
    pub liststate: ListState,
}

impl App {
    pub fn new() -> App {
        App {
            title: String::from("Task Manager"),
            quit: false,
            liststate: ListState::default(),
            highlighted: 0,
            tasklist: vec![
                Task {
                    text: "Learn more rust".to_string(),
                    done: true,
                },
                Task {
                    text: "Learn ratatui".to_string(),
                    done: false,
                },
                Task {
                    text: "Create a mini todo app".to_string(),
                    done: false,
                },
            ],
        }
    }

    pub fn quit(&mut self) {
        self.quit = true;
    }
}
