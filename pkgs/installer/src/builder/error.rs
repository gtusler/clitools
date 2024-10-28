use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum BuilderError {
    Placeholder,
}

impl Error for BuilderError {}

impl Display for BuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::Placeholder => String::from("Placeholder error"),
        };

        write!(f, "{}", msg)
    }
}
