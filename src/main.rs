use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem},
};
use std::io::stdout;

#[derive(Clone)]
struct Task {
    text: String,
    done: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let tasks = [
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
    ];

    loop {
        terminal.draw(|f| {
            let area = f.area();

            let items: Vec<ListItem> = tasks
                .iter()
                .map(|task| {
                    let checkbox = if task.done { "[x]" } else { "[ ]" };
                    let style = if task.done {
                        Style::default().fg(Color::Green)
                    } else {
                        Style::default()
                    };
                    ListItem::new(Line::from(vec![
                        Span::styled(checkbox, style),
                        Span::raw(" "),
                        Span::raw(&task.text),
                    ]))
                })
                .collect();

            let list = List::new(items)
                .block(
                    Block::default()
                        .title("tasks")
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::Cyan)),
                )
                .highlight_style(Style::default().add_modifier(Modifier::BOLD));

            f.render_widget(list, area);
        })?;

        #[allow(clippy::collapsible_if)]
        if event::poll(std::time::Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Char('Q') => break,
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
