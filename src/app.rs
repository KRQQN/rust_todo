pub struct App {
    pub title: String,
    pub quit: bool,
}

impl App {
    pub fn new() -> App {
        App {
            title: String::from("Task Manager"),
            quit: false,
        }
    }

    pub fn quit(&mut self) {
        self.quit = true;
    }
}
