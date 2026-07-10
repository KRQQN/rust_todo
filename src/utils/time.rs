use chrono::{DateTime, Duration, Local, NaiveDate};

#[derive(Clone, Copy, Debug, Default)]
pub struct ReminderParser;

impl ReminderParser {
    pub fn parse(input: &str) -> Option<DateTime<Local>> {
        let input = input.trim();
        if input.is_empty()
            || input.eq_ignore_ascii_case("none")
            || input.eq_ignore_ascii_case("no")
        {
            return None;
        }

        let now = Local::now();

        match input.to_lowercase().as_str() {
            "tomorrow" => return Some(now + Duration::days(1)),
            "today" => return Some(now),
            "next week" => return Some(now + Duration::weeks(1)),
            _ => {}
        }

        if let Some(dt) = Self::parse_relative(input, now) {
            return Some(dt);
        }

        Self::parse_absolute(input, now)
    }

    fn parse_relative(input: &str, now: DateTime<Local>) -> Option<DateTime<Local>> {
        let mut days = 0i64;
        let mut hours = 0i64;

        for word in input.split_whitespace() {
            let w = word.to_lowercase();
            if let Some(d) = w.strip_suffix('d') {
                if let Ok(n) = d.parse::<i64>() {
                    days += n;
                }
            } else if let Some(h) = w.strip_suffix('h') {
                if let Ok(n) = h.parse::<i64>() {
                    hours += n;
                }
            }
        }

        if days > 0 || hours > 0 {
            Some(now + Duration::days(days) + Duration::hours(hours))
        } else {
            None
        }
    }

    fn parse_absolute(input: &str, now: DateTime<Local>) -> Option<DateTime<Local>> {
        let formats = [
            "%Y-%m-%d %H:%M",
            "%d/%m/%Y %H:%M",
            "%d-%m-%Y %H:%M",
            "%Y-%m-%d",
            "%d/%m/%Y",
            "%d.%m.%Y",
        ];

        for fmt in formats {
            if let Ok(date) = NaiveDate::parse_from_str(input, fmt) {
                return date
                    .and_hms_opt(9, 0, 0)
                    .and_then(|naive_dt| naive_dt.and_local_timezone(Local).single());
            }

            if let Ok(dt) = DateTime::parse_from_str(input, fmt) {
                return Some(dt.with_timezone(&Local));
            }
        }

        None
    }
}
