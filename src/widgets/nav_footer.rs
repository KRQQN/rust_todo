use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
};

pub struct NavigationMenu;

impl NavigationMenu {
    pub fn render_main_menu(frame: &mut Frame, area: Rect) {
        let width = area.width as usize;

        let key_style = Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD);

        let action_style = Style::default().fg(Color::White);

        let controls = vec![
            (key_style, "[A]"),
            (action_style, " Add Task"),
            (key_style, " [↑↓]"),
            (action_style, " Navigate"),
            (key_style, " [Enter]"),
            (action_style, " Select"),
            (key_style, " [Q]"),
            (action_style, " Quit"),
        ];

        let mut spans = vec![];
        for (style, text) in controls {
            spans.push(Span::styled(text, style));
        }

        let content = if width < 65 {
            Line::from(vec![
                Span::styled("[A]", key_style),
                Span::styled(" Add  ", action_style),
                Span::styled("[↑↓]", key_style),
                Span::styled(" Nav  ", action_style),
                Span::styled("[⏎]", key_style),
                Span::styled(" Sel  ", action_style),
                Span::styled("[Q]", key_style),
                Span::styled(" Quit", action_style),
            ])
        } else {
            Line::from(spans)
        };

        frame.render_widget(
            Paragraph::new(content)
                .block(
                    Block::default()
                        .borders(Borders::TOP)
                        .border_type(BorderType::Plain)
                        .border_style(Style::default().fg(Color::Rgb(128, 128, 0))) // Dark yellow
                        .style(Style::default().bg(Color::Black)),
                )
                .style(Style::default().fg(Color::Yellow))
                .alignment(Alignment::Center),
            area,
        );
    }

    pub fn render_task_form_menu(frame: &mut Frame, area: Rect) {
        let key_style = Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD);

        let action_style = Style::default().fg(Color::White);

        let controls = vec![
            (key_style, "[Ctrl+S]"),
            (action_style, " Submit"),
            (key_style, " [Ctrl+C]"),
            (action_style, " Close"),
            (key_style, " [Esc]"),
            (action_style, " Cancel"),
        ];

        let mut spans = vec![];
        for (style, text) in controls {
            spans.push(Span::styled(text, style));
        }

        frame.render_widget(
            Paragraph::new(Line::from(spans))
                .block(
                    Block::default()
                        .borders(Borders::TOP)
                        .border_type(BorderType::Plain)
                        .border_style(Style::default().fg(Color::Rgb(128, 128, 0)))
                        .style(Style::default().bg(Color::Black)),
                )
                .style(Style::default().fg(Color::Yellow))
                .alignment(Alignment::Center),
            area,
        );
    }
}
