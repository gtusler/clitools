use sqlite::{Connection, State};
use std::{error::Error, fmt::Display};

pub struct Project {
    pub name: String,
}

impl Project {
    pub fn exists(connection: &Connection, name: &str) -> bool {
        let possible_project = Project::fetch_by_name(connection, name);

        possible_project.is_ok()
    }

    pub fn add(connection: &Connection, name: &str) -> Result<(), sqlite::Error> {
        let query = format!(
            r#"
INSERT INTO projects
(
    name
)
VALUES
(
    "{}"
)
"#,
            name
        );

        connection.execute(query)?;

        Ok(())
    }

    pub fn fetch_by_name(connection: &Connection, name: &str) -> Result<Self, sqlite::Error> {
        let query = format!(
            r#"
SELECT
    name
FROM
    projects
WHERE
    name="{}"
"#,
            name
        );

        let mut statement = connection.prepare(query)?;
        statement.next()?;

        let name = statement.read::<String, _>("name")?;

        Ok(Self { name })
    }

    pub fn fetch_all(connection: &Connection) -> Result<Vec<Self>, sqlite::Error> {
        let query = "SELECT name FROM projects";

        let mut statement = connection.prepare(query)?;
        let mut output: Vec<Project> = Vec::new();

        while let Ok(State::Row) = statement.next() {
            let name = statement.read::<String, _>("name")?;
            output.push(Project { name })
        }

        Ok(output)
    }

    pub fn fetch_running(connection: &Connection) -> Result<Project, ProjectError> {
        let query = "SELECT project FROM running";

        let mut statement = match connection.prepare(query) {
            Ok(s) => s,
            Err(e) => {
                return Err(ProjectError::SqliteError(e));
            }
        };

        if let Ok(State::Row) = statement.next() {
            let project_name = match statement.read::<String, _>("project") {
                Err(e) => {
                    return Err(ProjectError::SqliteError(e));
                }
                Ok(p) => p,
            };

            return Ok(Project { name: project_name });
        }

        Err(ProjectError::NotRunning)
    }
}

#[derive(Debug)]
pub enum ProjectError {
    NotRunning,
    SqliteError(sqlite::Error),
}

impl Error for ProjectError {}

impl Display for ProjectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::NotRunning => String::from("No running project"),
            Self::SqliteError(e) => format!("{}", e),
        };

        write!(f, "{}", msg)
    }
}
