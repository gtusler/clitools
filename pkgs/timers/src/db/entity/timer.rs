use super::{
    project::{Project, ProjectError},
    timing::{Timing, TimingError},
};
use sqlite::{Connection, State};
use std::{error::Error, fmt::Display};

pub struct Timer;

impl Timer {
    pub fn is_running(connection: &Connection) -> Result<bool, TimerError> {
        let query = "SELECT COUNT(*) FROM running";
        let mut statement = match connection.prepare(query) {
            Ok(s) => s,
            Err(e) => return Err(TimerError::SqliteError(e)),
        };

        if let Ok(State::Row) = statement.next() {
            let count = match statement.read::<i64, _>("COUNT(*)") {
                Ok(c) => c,
                Err(e) => return Err(TimerError::SqliteError(e)),
            };

            if count == 0 {
                return Ok(false);
            } else {
                return Ok(true);
            }
        }

        Err(TimerError::NoConnection)
    }

    pub fn start(connection: &Connection, project: &str, time: &str) -> sqlite::Result<()> {
        let project_query = format!(
            r#"
INSERT INTO running
(
    project
)
VALUES
(
    "{}"
)
"#,
            project
        );

        let timings_query = format!(
            r#"
INSERT INTO timings
(
    event,
    project,
    time
)
VALUES
(
    "start",
    "{}",
    "{}"
)
"#,
            project, time
        );

        connection.execute(project_query)?;
        connection.execute(timings_query)?;

        Ok(())
    }

    pub fn stop(connection: &Connection, time: &str) -> Result<(), TimerError> {
        let running_project = match Project::fetch_running(connection) {
            Ok(p) => p,
            Err(e) => return Err(TimerError::from(e)),
        };

        let running_query = format!(
            "DELETE FROM running WHERE project=\"{}\"",
            running_project.name
        );

        if let Err(e) = connection.execute(running_query) {
            return Err(TimerError::SqliteError(e));
        }

        let timings_query = format!(
            r#"
INSERT INTO timings
(
    event,
    project,
    time
)
VALUES
(
    "stop",
    "{}",
    "{}"
)
"#,
            running_project.name, time
        );

        match connection.execute(timings_query) {
            Ok(_) => Ok(()),
            Err(e) => Err(TimerError::SqliteError(e)),
        }
    }

    /// Remove the project from the running table
    /// and remove the started timer without an end
    pub fn abort(connection: &Connection) -> Result<(), TimerError> {
        let running_project = match Project::fetch_running(connection) {
            Ok(p) => p,
            Err(e) => return Err(TimerError::from(e)),
        };

        let running_query = format!(
            "DELETE FROM running WHERE project=\"{}\"",
            running_project.name
        );

        if let Err(e) = connection.execute(running_query) {
            return Err(TimerError::SqliteError(e));
        }

        match Timing::remove_unended(connection) {
            Ok(_) => Ok(()),
            Err(e) => Err(TimerError::from(e)),
        }
    }
}

#[derive(Debug)]
pub enum TimerError {
    NotRunning,
    SqliteError(sqlite::Error),
    TimingError(TimingError),
    NoConnection,
}

impl Error for TimerError {}

impl Display for TimerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::NotRunning => String::from("There are no running timers."),
            Self::SqliteError(e) => e.to_string(),
            Self::TimingError(e) => format!("Timing Error: {}", e),
            Self::NoConnection => String::from("Probably a sqlite connection error"),
        };

        write!(f, "{}", msg)
    }
}

impl From<TimingError> for TimerError {
    fn from(value: TimingError) -> Self {
        match value {
            TimingError::SqliteError(e) => TimerError::SqliteError(e),
            TimingError::DateTimeError(e) => TimerError::TimingError(TimingError::DateTimeError(e)),
            TimingError::NoMatch => TimerError::TimingError(TimingError::NoMatch),
            TimingError::UnableToRemove => TimerError::TimingError(TimingError::UnableToRemove),
        }
    }
}

impl From<ProjectError> for TimerError {
    fn from(value: ProjectError) -> Self {
        match value {
            ProjectError::NotRunning => TimerError::NotRunning,
            ProjectError::SqliteError(e) => TimerError::SqliteError(e),
        }
    }
}
