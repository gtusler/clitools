use std::fmt::Display;

#[derive(Default, Debug)]
pub enum OutputFormat {
    #[default]
    Print,
    Json,
    JsonPretty,
}

impl OutputFormat {
    pub fn from_raw(input: Option<&String>) -> OutputFormat {
        match input {
            None => Default::default(),
            Some(i) => match i.as_str() {
                "print" => OutputFormat::Print,
                "json" => OutputFormat::Json,
                "json-pretty" => OutputFormat::JsonPretty,
                _ => Default::default(),
            },
        }
    }
}

impl Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            OutputFormat::Print => "print",
            OutputFormat::Json => "json",
            OutputFormat::JsonPretty => "json-pretty",
        };
        write!(f, "{}", msg)
    }
}
