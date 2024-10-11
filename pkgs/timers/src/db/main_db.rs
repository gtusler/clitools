use crate::fs::main_dir::{main_dir, MainDirError};
use sqlite::Connection;
use std::{error::Error, fmt::Display, path::PathBuf};

/// Get the path to the main db
pub fn main_db_path() -> Result<PathBuf, MainDbError> {
    let mut dir = match main_dir() {
        Err(e) => {
            return Err(MainDbError::DirError(e));
        }
        Ok(d) => d,
    };

    dir.push("main.db");

    Ok(dir)
}

/// Get a connection to the main db
pub fn connect_main_db() -> Result<Connection, MainDbError> {
    let db_path = match main_db_path() {
        Err(e) => {
            return Err(e);
        }
        Ok(p) => p,
    };

    match sqlite::open(db_path) {
        Err(e) => Err(MainDbError::SqliteError(e)),
        Ok(con) => Ok(con),
    }
}

#[derive(Debug)]
pub enum MainDbError {
    DirError(MainDirError),
    SqliteError(sqlite::Error),
}

impl Error for MainDbError {}

impl Display for MainDbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            MainDbError::DirError(dir_error) => {
                format!("Main dir error: {}", dir_error)
            }
            MainDbError::SqliteError(sqlite_error) => {
                format!("Sqlite error: {}", sqlite_error)
            }
        };

        write!(f, "{}", msg)
    }
}
