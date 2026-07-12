use chrono::{DateTime, Local};
use ratatui::{
    style::{Color, Style},
    text::Span,
};

#[derive(Clone, Copy, Debug, Default)]
pub struct ReminderTimeFormatter;

impl ReminderTimeFormatter {
    pub fn format_reminder(
        reminder: Option<DateTime<Local>>,
        completed_at: Option<DateTime<Local>>,
        is_done: bool,
    ) -> Option<Span<'static>> {
        let reminder = reminder?;

        if is_done {
            Self::format_completed(reminder, completed_at)
        } else {
            Self::format_remaining(reminder)
        }
    }

    fn format_completed(
        reminder: DateTime<Local>,
        completed_at: Option<DateTime<Local>>,
    ) -> Option<Span<'static>> {
        let completed_at = completed_at?;
        let diff = reminder.signed_duration_since(completed_at);

        let (text, color) = Self::completion_text_and_color(diff);

        Some(Self::styled_span(text, color))
    }

    fn format_remaining(reminder: DateTime<Local>) -> Option<Span<'static>> {
        let duration = reminder.signed_duration_since(Local::now());

        let text = Self::remaining_text(duration);
        let color = Self::remaining_color(duration);

        Some(Self::styled_span(text, color))
    }

    fn completion_text_and_color(diff: chrono::Duration) -> (String, Color) {
        if diff.num_seconds() > 0 {
            (Self::signed_duration_text(diff, '+'), Color::Green)
        } else if diff.num_seconds() < 0 {
            (Self::signed_duration_text(diff.abs(), '-'), Color::Red)
        } else {
            ("".to_string(), Color::Yellow)
        }
    }

    fn signed_duration_text(duration: chrono::Duration, sign: char) -> String {
        let days = duration.num_days();
        let hours = duration.num_hours();
        let minutes = duration.num_minutes();

        if hours > 0 {
            if hours >= 24 {
                format!(" {}{}d {}h", sign, days, hours % 24)
            } else {
                format!(" {}{}h {}m", sign, hours, minutes % 60)
            }
        } else {
            format!(" {}{}m", sign, duration.num_minutes())
        }
    }

    fn remaining_text(duration: chrono::Duration) -> String {
        if duration.num_days() > 0 {
            Self::days_text(duration)
        } else if duration.num_hours() > 0 {
            Self::hours_text(duration)
        } else if duration.num_minutes() > 0 {
            Self::minutes_text(duration)
        } else {
            "".to_string()
        }
    }

    fn days_text(duration: chrono::Duration) -> String {
        format!(" ({}d)", duration.num_days())
    }

    fn hours_text(duration: chrono::Duration) -> String {
        let hours = duration.num_hours();
        let minutes = duration.num_minutes() % 60;

        if hours < 6 {
            format!(" ({}h {}m)", hours, minutes)
        } else {
            format!(" ({}h)", hours)
        }
    }

    fn minutes_text(duration: chrono::Duration) -> String {
        format!(" ({}m)", duration.num_minutes())
    }

    fn remaining_color(duration: chrono::Duration) -> Color {
        if duration.num_hours() <= 24 {
            Color::Red
        } else if duration.num_days() <= 7 {
            Color::Yellow
        } else {
            Color::LightCyan
        }
    }

    fn styled_span(text: String, color: Color) -> Span<'static> {
        Span::styled(text, Style::default().fg(color))
    }
}
