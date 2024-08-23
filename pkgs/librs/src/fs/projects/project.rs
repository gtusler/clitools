use std::fmt::Display;

pub struct Project {
    pub name: String,
    pub config_file_path: String,
}

impl Project {
    pub fn from_path(path: String) -> Project {
        Project {
            name: String::from("unknown"),
            config_file_path: path,
        }
    }
}

impl Display for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
