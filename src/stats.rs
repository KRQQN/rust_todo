use chrono::Local;
use ratatui::prelude::*;
use ratatui::widgets::Paragraph;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders},
};

use crate::widgets::task::Task;

pub struct TaskStats {
    pending_task_count: u32,
    overdue_completed_task_count: u32,
    completed_task_count: u32,
    overdue_pending_task_count: u32,
}

impl TaskStats {
    pub fn new() -> Self {
        TaskStats {
            pending_task_count: 0,
            completed_task_count: 0,
            overdue_completed_task_count: 0,
            overdue_pending_task_count: 0,
        }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let outer_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow))
            .border_type(BorderType::Thick)
            .title(" | Statistics | ")
            .title_alignment(Alignment::Center)
            .style(Style::default().bg(Color::Black));

        frame.render_widget(&outer_block, area);
        let inner_area = outer_block.inner(area);

        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(35), Constraint::Percentage(65)])
            .split(inner_area.inner(Margin::new(2, 1)));

        // LEFT INNER SECTION
        let left_stats = vec![
            Line::from(vec![
                Span::styled("Total Tasks: ", Style::default().fg(Color::White)),
                Span::styled(
                    (self.completed_task_count + self.pending_task_count).to_string(),
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(vec![Span::styled(" ", Style::default())]),
            Line::from(vec![
                Span::styled("Completed: ", Style::default().fg(Color::White)),
                Span::styled(
                    self.completed_task_count.to_string(),
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(vec![
                Span::styled(" └─ Overdue: ", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    self.overdue_completed_task_count.to_string(),
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(vec![Span::styled(" ", Style::default())]),
            Line::from(vec![
                Span::styled("Pending:   ", Style::default().fg(Color::White)),
                Span::styled(
                    self.pending_task_count.to_string(),
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(vec![
                Span::styled(" └─ Overdue: ", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    self.overdue_pending_task_count.to_string(),
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(vec![Span::styled(" ", Style::default())]),
            Line::from(vec![
                Span::styled("Completion: ", Style::default().fg(Color::White)),
                Span::styled(
                    format!("{}%", self.completion_rate()),
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(vec![
                Span::styled("Overdue Rate:  ", Style::default().fg(Color::White)),
                Span::styled(
                    format!("{}%", self.overdue_rate()), // of pending tasks
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                ),
            ]),
        ];

        let left_paragraph = Paragraph::new(left_stats)
            .block(
                Block::default()
                    .borders(Borders::RIGHT)
                    .border_style(Style::default().fg(Color::DarkGray)),
            )
            .alignment(Alignment::Left);

        // RIGHT INNER SECTION
        let right_paragraph = Paragraph::new("")
            .block(
                Block::default()
                    .borders(Borders::LEFT)
                    .border_style(Style::default().fg(Color::DarkGray)),
            )
            .alignment(Alignment::Left);

        frame.render_widget(left_paragraph, layout[0]);
        frame.render_widget(right_paragraph, layout[1]);
    }

    pub fn get_pending_count(&self) -> u32 {
        self.pending_task_count
    }

    pub fn get_completed_count(&self) -> u32 {
        self.completed_task_count
    }

    pub fn get_overdue_count(&self) -> u32 {
        self.overdue_pending_task_count
    }

    fn overdue_rate(&self) -> u32 {
        if self.pending_task_count == 0 {
            0
        } else {
            ((self.overdue_pending_task_count as f32 / self.pending_task_count as f32) * 100.0)
                as u32
        }
    }

    fn completion_rate(&self) -> u32 {
        let total = self.completed_task_count + self.pending_task_count;
        if total == 0 {
            0
        } else {
            (self.completed_task_count as f32 / total as f32 * 100.0) as u32
        }
    }

    pub fn sync_task_stats(&mut self, tasklist: &Vec<Task>) {
        self.pending_task_count = 0;
        self.completed_task_count = 0;
        self.overdue_pending_task_count = 0;
        self.overdue_completed_task_count = 0;

        for task in tasklist {
            if task.done {
                self.completed_task_count += 1;

                if task.completed_at > task.reminder {
                    self.overdue_completed_task_count += 1;
                }
            } else {
                self.pending_task_count += 1;

                if let Some(rem) = &task.reminder {
                    if rem < &Local::now() {
                        self.overdue_pending_task_count += 1;
                    }
                }
            }
        }
    }
}

impl Default for TaskStats {
    fn default() -> Self {
        Self::new()
    }
}
