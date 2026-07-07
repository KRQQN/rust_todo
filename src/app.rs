use ratatui::widgets::ListState;

use crate::widgets::{io::Io, tasklist::Task};

pub struct App {
    pub title: String,
    pub quit: bool,
    pub input_mode: InputMode,
    pub io: Io,
    pub tasklist: Vec<Task>,
    pub highlighted: usize,
    pub liststate: ListState,
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
