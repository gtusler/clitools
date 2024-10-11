use std::{error::Error, fmt::Display};

use super::main_db::MainDbError;

#[derive(Debug)]
pub enum DbError {
    MainDbError(MainDbError),
    SqliteError(sqlite::Error),
}

impl Error for DbError {}

impl Display for DbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::MainDbError(e) => {
                format!("Main db error: {}", e)
            }
            Self::SqliteError(e) => {
                format!("Sqlite error: {}", e)
            }
        };

        write!(f, "{}", msg)
    }
}
