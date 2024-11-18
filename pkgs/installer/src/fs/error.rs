use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum FsError {
    FileNotFound,
}

impl Error for FsError {}

impl Display for FsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::FileNotFound => "File Not Found",
        };
        write!(f, "{}", msg)
    }
}
