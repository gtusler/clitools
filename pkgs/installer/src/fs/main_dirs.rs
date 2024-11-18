use std::{error::Error, fmt::Display, path::PathBuf};

pub fn main_dir() -> Result<PathBuf, DirError> {
    match dirs::data_dir() {
        Some(mut dir) => {
            dir.push("clit");
            if dir.exists() {
                return Ok(dir);
            }
            match std::fs::create_dir(&dir) {
                Ok(_) => Ok(dir),
                Err(e) => Err(DirError::UnableToCreate(e)),
            }
        }
        None => Err(DirError::NoDataDir),
    }
}

pub fn exe_dir() -> Result<PathBuf, DirError> {
    match dirs::executable_dir() {
        Some(mut dir) => {
            dir.push("clit");
            if dir.exists() {
                return Ok(dir);
            }
            match std::fs::create_dir(&dir) {
                Ok(_) => Ok(dir),
                Err(e) => Err(DirError::UnableToCreate(e)),
            }
        }
        None => Err(DirError::NoExecutableDir),
    }
}

#[derive(Debug)]
pub enum DirError {
    NoDataDir,
    NoExecutableDir,
    UnableToCreate(std::io::Error),
}

impl Error for DirError {}

impl Display for DirError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            DirError::NoDataDir => String::from("No data dir"),
            DirError::NoExecutableDir => String::from("No executable dir"),
            DirError::UnableToCreate(io_error) => format!("Unable to create dir: {}", io_error),
        };

        write!(f, "{}", msg)
    }
}
