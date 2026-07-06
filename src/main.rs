pub mod app;
pub mod event;
pub mod tui;
pub mod ui;
pub mod update;

use app::App;
use color_eyre::eyre::Result;
use event::{Event, EventHandler};
use ratatui::crossterm::terminal::enable_raw_mode;
use ratatui::{Terminal, backend::CrosstermBackend};
use tui::Tui;
use update::update;

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut app = App::new();
    let terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);

    tui.enter()?;

    while !app.quit {
        tui.draw(&mut app)?;
        match tui.events.next()? {
            Event::Tick => {}
            Event::Key(key_event) => update(&mut app, key_event),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        };
    }
    tui.exit()?;
    Ok(())
}
