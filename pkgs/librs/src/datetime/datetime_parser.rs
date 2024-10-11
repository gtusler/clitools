use chrono::{DateTime, NaiveDateTime, Utc};

pub struct DateTimeParser;

impl DateTimeParser {
    pub fn from_ymd_hms(input: &str) -> Result<DateTime<Utc>, chrono::ParseError> {
        let naive = match NaiveDateTime::parse_from_str(input, "%Y-%m-%d %H:%M:%S") {
            Ok(dt) => dt,
            Err(e) => return Err(e),
        };

        Ok(DateTime::from_naive_utc_and_offset(naive, Utc))
    }
}
