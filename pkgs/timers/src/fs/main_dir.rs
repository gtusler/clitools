use std::{error::Error, fmt::Display, path::PathBuf};

/**
 * Get the path to the main dir.
 * Creates the main dir if it doesn't exist.
 */
pub fn main_dir() -> Result<PathBuf, MainDirError> {
    let data_dir = dirs::data_dir();
    match data_dir {
        Some(mut dir) => {
            dir.push("timer-rs");

            if dir.exists() {
                return Ok(dir);
            }

            match std::fs::create_dir(&dir) {
                Ok(_) => {
                    Ok(dir)
                }
                Err(e) => {
                    Err(MainDirError::UnableToCreate(e))
                }
            }
        }
        None => {
            Err(MainDirError::NoDataDir)
        }
    }
}

#[derive(Debug)]
pub enum MainDirError {
    NoDataDir,
    UnableToCreate(std::io::Error),
}

impl Error for MainDirError {}

impl Display for MainDirError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            MainDirError::NoDataDir => String::from("No data dir"),
            MainDirError::UnableToCreate(io_error) => {
                format!("Unable to create main dir: {}", io_error)
            }
        };

        write!(f, "{}", msg)
    }
}
