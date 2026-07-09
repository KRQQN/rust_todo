use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Flex, Layout, Margin, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Padding, Paragraph},
};

use crate::widgets::io::Io;

pub struct FormField {
    pub label: String,
    pub io: Io,
    pub value: String,
    pub max_length: usize,
    pub box_width: u16,
}

pub struct AddTaskForm {
    pub fields: Vec<FormField>,
    pub active: usize,
    pub show: bool,
}

impl AddTaskForm {
    pub fn new() -> Self {
        Self {
            fields: vec![
                FormField {
                    label: "Title".into(),
                    io: Io::new(),
                    value: String::new(),
                    max_length: 80,
                    box_width: 60,
                },
                FormField {
                    label: "Description".into(),
                    io: Io::new(),
                    value: String::new(),
                    max_length: 300,
                    box_width: 60,
                },
            ],
            active: 0,
            show: true,
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        if !self.show {
            return;
        }

        let popup_area = area.centered(Constraint::Length(60), Constraint::Length(100));

        frame.render_widget(Clear, popup_area);

        let outer_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow))
            .title("Add New Task")
            .padding(Padding::new(2, 2, 1, 1));

        let inner_area = outer_block.inner(popup_area);
        frame.render_widget(outer_block, popup_area);

        let field_areas = Layout::default()
            .direction(Direction::Vertical)
            .flex(Flex::Center)
            .spacing(1)
            .constraints(vec![Constraint::Length(3); 3])
            .split(inner_area);

        for (i, field) in self.fields.iter().enumerate() {
            let is_active = i == self.active;

            let input_block = Block::default()
                .borders(Borders::ALL)
                .border_style(if is_active {
                    Style::default().fg(Color::Yellow)
                } else {
                    Style::default().fg(Color::DarkGray)
                })
                .title(Line::from(vec![
                    Span::styled(&field.label, Style::default().fg(Color::Gray)),
                    Span::raw(format!(" ({}/{})", field.io.input.len(), field.max_length)),
                ]));

            let input_layout = Layout::default()
                .direction(Direction::Horizontal)
                .flex(Flex::Center)
                .constraints([Constraint::Length(field.box_width + 2)])
                .split(field_areas[i]);

            let input_area = input_layout[0];

            frame.render_widget(input_block, input_area);

            let display_text = if field.io.input.is_empty() && !field.value.is_empty() {
                field.value.as_str()
            } else {
                field.io.input.as_str()
            };

            let paragraph = Paragraph::new(display_text).style(Style::default().fg(if is_active {
                Color::Yellow
            } else {
                Color::White
            }));

            frame.render_widget(paragraph, input_area.inner(Margin::new(1, 1)));
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> Option<String> {
        if !self.show {
            return None;
        }

        let field = &mut self.fields[self.active];

        match key.code {
            KeyCode::Enter => {
                if !field.io.input.trim().is_empty() {
                    field.value = field.io.input.clone();
                    field.io.input.clear();
                    field.io.character_index = 0;
                }

                self.active = (self.active + 1) % self.fields.len();
                if self.active == 0 {
                    return Some("submitted".into());
                }
                None
            }

            KeyCode::Tab | KeyCode::Down => {
                self.active = (self.active + 1) % self.fields.len();
                None
            }

            KeyCode::BackTab | KeyCode::Up => {
                self.active = if self.active == 0 {
                    self.fields.len() - 1
                } else {
                    self.active - 1
                };
                None
            }

            KeyCode::Esc => {
                self.show = false;
                None
            }

            _ => {
                if field.io.input.len() < field.max_length
                    || matches!(
                        key.code,
                        KeyCode::Backspace | KeyCode::Left | KeyCode::Right
                    )
                {
                    field.io.handle_key(key);
                }
                None
            }
        }
    }
}
