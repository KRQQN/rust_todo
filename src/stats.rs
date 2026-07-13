use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};

pub struct TaskStats {
    pending_task_count: u32,
    completed_task_count: u32,
    overdue_count: u32,
}

impl TaskStats {
    pub fn new() -> Self {
        TaskStats {
            pending_task_count: 0,
            completed_task_count: 0,
            overdue_count: 0,
        }
    }

    pub fn render(frame: &mut Frame, area: Rect) {
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
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(inner_area);

        // LEFT INNER SECTION
        let left_block = Block::default()
            .borders(Borders::RIGHT)
            .border_style(Style::default().fg(Color::Yellow))
            .border_type(BorderType::Plain)
            .title_alignment(Alignment::Center)
            .style(Style::default().bg(Color::Black));

        let paragraph = Paragraph::new("").style(Style::default()).block(left_block);

        // RIGHT INNER SECTION
        let right_block = Block::default()
            .borders(Borders::LEFT)
            .border_style(Style::default().fg(Color::Yellow))
            .border_type(BorderType::Plain)
            .title_alignment(Alignment::Center)
            .style(Style::default().bg(Color::Black));

        let paragraph2 = Paragraph::new("")
            .style(Style::default())
            .block(right_block);

        frame.render_widget(paragraph, layout[0]);
        frame.render_widget(paragraph2, layout[1]);
    }

    pub fn get_pending_count(&self) -> u32 {
        self.pending_task_count
    }

    pub fn get_completed_count(&self) -> u32 {
        self.completed_task_count
    }

    pub fn get_overdue_count(&self) -> u32 {
        self.overdue_count
    }
}

impl Default for TaskStats {
    fn default() -> Self {
        Self::new()
    }
}
