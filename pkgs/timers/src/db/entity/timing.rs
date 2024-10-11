use crate::enums::timer_event::TimerEvent;
use chrono::{DateTime, Utc};
use librs::datetime::datetime_parser::DateTimeParser;
use sqlite::{Connection, State, Statement};
use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Timing {
    pub start: DateTime<Utc>,
    pub stop: DateTime<Utc>,
    pub project: String,
}

#[derive(Clone)]
pub struct TimingPartial {
    pub event: TimerEvent,
    pub project: String,
    pub time: DateTime<Utc>,
}

impl Timing {
    /// Get all the timings for a project
    pub fn fetch_project_all(
        connection: &Connection,
        project: &str,
    ) -> Result<Vec<Timing>, TimingError> {
        let query = format!(
            r#"
SELECT
    event,
    time
FROM
    timings
WHERE
    project="{}"
"#,
            project
        );

        let statement = match connection.prepare(query) {
            Ok(s) => s,
            Err(e) => return Err(TimingError::SqliteError(e)),
        };
        let partials = gather_partials_from_statement(statement, project)?;

        gather_partial_pairs(partials)
    }

    /// Get the latest timing, irregardless of which project it belongs to
    pub fn fetch_latest(connection: &Connection) -> Result<Timing, TimingError> {
        let query = r#"
SELECT
    event,
    project,
    time
FROM
    timings
ORDER BY id
LIMIT 2
"#;

        let statement = match connection.prepare(query) {
            Ok(s) => s,
            Err(e) => return Err(TimingError::SqliteError(e)),
        };

        let project = match statement.read::<String, _>("project") {
            Ok(p) => p,
            Err(e) => return Err(TimingError::SqliteError(e)),
        };

        let partials = gather_partials_from_statement(statement, &project)?;
        let paired = gather_partial_pairs(partials)?;

        if paired.len() == 0 {
            return Err(TimingError::NoMatch);
        }

        match paired.get(0) {
            Some(timing) => Ok(timing.clone()),
            None => Err(TimingError::NoMatch),
        }
    }

    /// Remove the latest timing which doesn't have an end
    pub fn remove_unended(connection: &Connection) -> Result<(), TimingError> {
        let query = "SELECT id, event FROM timings ORDER BY id DESC LIMIT 1";

        let mut statement = match connection.prepare(query) {
            Ok(s) => s,
            Err(e) => return Err(TimingError::SqliteError(e)),
        };

        if let Ok(State::Row) = statement.next() {
            let id = match statement.read::<i64, _>("id") {
                Ok(i) => i,
                Err(e) => return Err(TimingError::SqliteError(e)),
            };
            let event_raw = match statement.read::<String, _>("event") {
                Ok(ev) => ev,
                Err(e) => return Err(TimingError::SqliteError(e)),
            };
            let event = TimerEvent::from_str(&event_raw);

            if event == TimerEvent::Start {
                Timing::remove_by_id(connection, &id)?;
                return Ok(());
            }
        }

        Err(TimingError::UnableToRemove)
    }

    pub fn remove_by_id(connection: &Connection, id: &i64) -> Result<(), TimingError> {
        let query = format!("DELETE FROM timings WHERE id={}", id);

        match connection.execute(query) {
            Ok(_) => Ok(()),
            Err(e) => Err(TimingError::SqliteError(e)),
        }
    }
}

fn gather_partials_from_statement(
    mut statement: Statement,
    project: &str,
) -> Result<Vec<TimingPartial>, TimingError> {
    let mut rows: Vec<TimingPartial> = Vec::new();
    while let Ok(State::Row) = statement.next() {
        let event_raw = match statement.read::<String, _>("event") {
            Ok(ev) => ev,
            Err(e) => return Err(TimingError::SqliteError(e)),
        };
        let event = TimerEvent::from_str(&event_raw);
        let time_raw = match statement.read::<String, _>("time") {
            Ok(t) => t,
            Err(e) => return Err(TimingError::SqliteError(e)),
        };
        let time = match DateTimeParser::from_ymd_hms(&time_raw) {
            Ok(t) => t,
            Err(e) => return Err(TimingError::DateTimeError(e)),
        };

        rows.push(TimingPartial {
            event,
            project: project.to_string(),
            time,
        });
    }

    Ok(rows)
}

fn gather_partial_pairs(partials: Vec<TimingPartial>) -> Result<Vec<Timing>, TimingError> {
    let mut timings: Vec<Timing> = Vec::new();
    let mut floating_partial: Option<TimingPartial> = None;

    for partial in partials {
        if partial.event == TimerEvent::Start {
            floating_partial = Some(partial.clone());
        }

        if partial.event == TimerEvent::Stop {
            if let None = floating_partial {
                return Err(TimingError::NoMatch);
            }

            let time_start = floating_partial.unwrap().clone().time;
            let time_stop = partial.time;

            timings.push(Timing {
                start: time_start,
                stop: time_stop,
                project: partial.project,
            });

            floating_partial = None;
        }
    }

    Ok(timings)
}

#[derive(Debug)]
pub enum TimingError {
    SqliteError(sqlite::Error),
    DateTimeError(chrono::ParseError),
    NoMatch,
    UnableToRemove,
}

impl Error for TimingError {}

impl Display for TimingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::SqliteError(e) => e.to_string(),
            Self::DateTimeError(e) => e.to_string(),
            Self::NoMatch => "Various reasons, if encountered restructure source.".to_string(),
            Self::UnableToRemove => "Unable to remove timing for some reason.".to_string(),
        };

        write!(f, "{}", msg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod gather_partial_pairs {
        use chrono::TimeZone;

        use super::*;

        #[test]
        fn it_handles_empty_list() {
            assert_eq!(gather_partial_pairs(Vec::new()).unwrap(), Vec::new());
        }

        #[test]
        fn it_handles_good_pair() {
            let dt_start = Utc.with_ymd_and_hms(2024, 9, 7, 18, 30, 0).unwrap();
            let dt_stop = Utc.with_ymd_and_hms(2024, 9, 7, 18, 50, 0).unwrap();
            assert_eq!(
                gather_partial_pairs(vec![
                    TimingPartial {
                        event: TimerEvent::Start,
                        project: String::from("testTHIS"),
                        time: dt_start,
                    },
                    TimingPartial {
                        event: TimerEvent::Stop,
                        project: String::from("testTHIS"),
                        time: dt_stop,
                    }
                ])
                .unwrap(),
                vec![Timing {
                    start: dt_start,
                    stop: dt_stop,
                    project: String::from("testTHIS"),
                }]
            );
        }

        #[test]
        fn it_handles_good_pairs() {
            let dt_start_1 = Utc.with_ymd_and_hms(2024, 9, 7, 18, 30, 0).unwrap();
            let dt_stop_1 = Utc.with_ymd_and_hms(2024, 9, 7, 18, 50, 0).unwrap();
            let dt_start_2 = Utc.with_ymd_and_hms(2024, 9, 7, 19, 00, 0).unwrap();
            let dt_stop_2 = Utc.with_ymd_and_hms(2024, 9, 7, 19, 30, 0).unwrap();

            assert_eq!(
                gather_partial_pairs(vec![
                    TimingPartial {
                        event: TimerEvent::Start,
                        project: String::from("testTHIS"),
                        time: dt_start_1,
                    },
                    TimingPartial {
                        event: TimerEvent::Stop,
                        project: String::from("testTHIS"),
                        time: dt_stop_1,
                    },
                    TimingPartial {
                        event: TimerEvent::Start,
                        project: String::from("testTHIS"),
                        time: dt_start_2,
                    },
                    TimingPartial {
                        event: TimerEvent::Stop,
                        project: String::from("testTHIS"),
                        time: dt_stop_2,
                    },
                ])
                .unwrap(),
                vec![
                    Timing {
                        start: dt_start_1,
                        stop: dt_stop_1,
                        project: String::from("testTHIS"),
                    },
                    Timing {
                        start: dt_start_2,
                        stop: dt_stop_2,
                        project: String::from("testTHIS"),
                    },
                ]
            );
        }

        #[test]
        fn it_handles_unmatched_pairs_1() {
            // the intended behavior is to simply ignore the unmatched values

            let dt_start_1 = Utc.with_ymd_and_hms(2024, 9, 7, 18, 30, 0).unwrap();
            let dt_start_2 = Utc.with_ymd_and_hms(2024, 9, 7, 19, 00, 0).unwrap();
            let dt_stop_2 = Utc.with_ymd_and_hms(2024, 9, 7, 19, 30, 0).unwrap();

            assert_eq!(
                gather_partial_pairs(vec![
                    TimingPartial {
                        event: TimerEvent::Start,
                        project: String::from("testTHIS"),
                        time: dt_start_1,
                    },
                    TimingPartial {
                        event: TimerEvent::Start,
                        project: String::from("testTHIS"),
                        time: dt_start_2,
                    },
                    TimingPartial {
                        event: TimerEvent::Stop,
                        project: String::from("testTHIS"),
                        time: dt_stop_2,
                    },
                ])
                .unwrap(),
                vec![Timing {
                    start: dt_start_2,
                    stop: dt_stop_2,
                    project: String::from("testTHIS"),
                },]
            );
        }

        #[test]
        fn it_handles_unmatched_pairs_2() {
            // the intended behavior is to simply ignore the unmatched values

            let dt_start_1 = Utc.with_ymd_and_hms(2024, 9, 7, 18, 30, 0).unwrap();
            let dt_stop_1 = Utc.with_ymd_and_hms(2024, 9, 7, 18, 50, 0).unwrap();
            let dt_start_2 = Utc.with_ymd_and_hms(2024, 9, 7, 19, 00, 0).unwrap();

            assert_eq!(
                gather_partial_pairs(vec![
                    TimingPartial {
                        event: TimerEvent::Start,
                        project: String::from("testTHIS"),
                        time: dt_start_1,
                    },
                    TimingPartial {
                        event: TimerEvent::Stop,
                        project: String::from("testTHIS"),
                        time: dt_stop_1,
                    },
                    TimingPartial {
                        event: TimerEvent::Start,
                        project: String::from("testTHIS"),
                        time: dt_start_2,
                    },
                ])
                .unwrap(),
                vec![Timing {
                    start: dt_start_1,
                    stop: dt_stop_1,
                    project: String::from("testTHIS"),
                },]
            );
        }
    }
}
