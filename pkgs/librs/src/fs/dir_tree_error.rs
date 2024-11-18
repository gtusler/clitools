use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum TreeError {
    Unknown,
    IoError(std::io::Error),
    LeafError(LeafError),
}

impl Error for TreeError {}

impl Display for TreeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::IoError(err) => format!("io error: {}", err),
            Self::LeafError(e) => format!("LeafError: {}", e),
            Self::Unknown => String::from("Unknown error"),
        };

        write!(f, "{}", msg)
    }
}

#[derive(Debug)]
pub enum LeafError {
    IoError(std::io::Error),
    NoName,
    UnsupportedType,
}

impl Error for LeafError {}

impl Display for LeafError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::IoError(err) => format!("io error: {}", err),
            Self::NoName => String::from("No name"),
            Self::UnsupportedType => String::from("Unsupported type"),
        };

        write!(f, "{}", msg)
    }
}
