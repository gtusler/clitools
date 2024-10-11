use chrono::{DateTime, Utc};

pub struct DateTimeFormatter;

// https://docs.rs/chrono/0.4.38/chrono/format/strftime/index.html

impl DateTimeFormatter {
    pub fn ymd_hms(input: &DateTime<Utc>) -> String {
        format!("{}", input.format("%Y-%m-%d %H:%M:%S"))
    }

    pub fn hms(input: &DateTime<Utc>) -> String {
        format!("{}", input.format("%H:%M:%S"))
    }
}
