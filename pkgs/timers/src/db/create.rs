use super::{error::DbError, main_db::connect_main_db};

pub fn create_db() -> Result<(), DbError> {
    let connection = match connect_main_db() {
        Err(e) => {
            return Err(DbError::MainDbError(e));
        }
        Ok(con) => con,
    };

    let query = r#"
CREATE TABLE IF NOT EXISTS projects (
    name  TEXT  UNIQUE NOT NULL PRIMARY KEY
);

CREATE TABLE IF NOT EXISTS running (
    id       INTEGER  PRIMARY KEY,
    project  TEXT     NOT NULL,
    FOREIGN KEY (project) REFERENCES projects(name) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS timings (
    id       INTEGER  PRIMARY KEY,
    event    TEXT     NOT NULL,
    project  TEXT     NOT NULL,
    time     TEXT     NOT NULL,
    FOREIGN KEY (project) REFERENCES projects(name) ON DELETE CASCADE
);
"#;

    match connection.execute(query) {
        Ok(_) => Ok(()),
        Err(e) => Err(DbError::SqliteError(e)),
    }
}
