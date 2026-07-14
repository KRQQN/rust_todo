use chrono::{DateTime, Datelike, Local, TimeDelta};
use ratatui::prelude::*;
use ratatui::widgets::Paragraph;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders},
};

use crate::utils::time_formatter::TimeFormatter;
use crate::widgets::task::Task;

pub struct TaskStats {
    pending_task_count: i32,
    overdue_completed_task_count: i32,
    completed_task_count: i32,
    overdue_pending_task_count: i32,
    this_week_completed: i32,
    due_this_week: i32,
    this_week_completed_late: i32,
    time_balance: TimeDelta,
}

impl TaskStats {
    pub fn new() -> Self {
        TaskStats {
            pending_task_count: 0,
            completed_task_count: 0,
            overdue_completed_task_count: 0,
            overdue_pending_task_count: 0,
            this_week_completed: 0,
            this_week_completed_late: 0,
            due_this_week: 0,
            time_balance: TimeDelta::zero(),
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
                Span::styled("This Week: ", Style::default().fg(Color::White))
                    .add_modifier(Modifier::BOLD),
            ]),
            Line::from(vec![
                Span::styled(" └─ Completed: ", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    self.this_week_completed.to_string(),
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(vec![
                Span::styled(" └─ Pending:   ", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    self.due_this_week.to_string(),
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(vec![
                Span::styled(" └─ Overdued: ", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    self.this_week_completed_late.to_string(),
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(vec![Span::styled(" ", Style::default())]),
            Line::from(vec![
                Span::styled("Total: ", Style::default().fg(Color::White))
                    .add_modifier(Modifier::BOLD),
            ]),
            Line::from(vec![
                Span::styled(" └─ Completed: ", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    self.completed_task_count.to_string(),
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(vec![
                Span::styled(" └─ Pending: ", Style::default().fg(Color::DarkGray)),
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
                Span::styled("Completion Rate: ", Style::default().fg(Color::White)),
                Span::styled(
                    format!("{}%", self.completion_rate()),
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(vec![
                Span::styled("Overdue Rate: ", Style::default().fg(Color::White)),
                Span::styled(
                    format!("{}%", self.overdue_rate()), // of pending tasks
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(vec![
                Span::styled("Time Balance:", Style::default().fg(Color::White)),
                TimeFormatter::display_time_balance(self.time_balance),
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

    pub fn get_pending_count(&self) -> i32 {
        self.pending_task_count
    }

    pub fn get_completed_count(&self) -> i32 {
        self.completed_task_count
    }

    pub fn get_overdue_count(&self) -> i32 {
        self.overdue_pending_task_count
    }

    fn overdue_rate(&self) -> i32 {
        if self.pending_task_count == 0 {
            0
        } else {
            ((self.overdue_pending_task_count as f32 / self.pending_task_count as f32) * 100.0)
                as i32
        }
    }

    fn completion_rate(&self) -> i32 {
        let total = self.completed_task_count + self.pending_task_count;
        if total == 0 {
            0
        } else {
            (self.completed_task_count as f32 / total as f32 * 100.0) as i32
        }
    }

    fn is_this_week(&self, date: DateTime<Local>) -> bool {
        date.iso_week() == Local::now().iso_week()
    }

    pub fn sync_task_stats(&mut self, tasklist: &Vec<Task>) {
        self.pending_task_count = 0;
        self.completed_task_count = 0;
        self.overdue_pending_task_count = 0;
        self.overdue_completed_task_count = 0;
        self.this_week_completed = 0;
        self.due_this_week = 0;
        self.time_balance = TimeDelta::zero();

        for task in tasklist {
            if task.done {
                self.completed_task_count += 1;

                if task.completed_at.is_some_and(|dt| self.is_this_week(dt)) {
                    self.this_week_completed += 1;
                }

                if let (Some(completed_at), Some(reminder)) = (task.completed_at, task.reminder) {
                    self.time_balance += reminder.signed_duration_since(completed_at);

                    if self.is_this_week(completed_at)
                        && self.is_this_week(reminder)
                        && completed_at > reminder
                    {
                        self.this_week_completed_late += 1;
                    }
                }
            } else {
                self.pending_task_count += 1;

                if task.reminder.is_some_and(|dt| self.is_this_week(dt)) {
                    self.due_this_week += 1;
                }

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
