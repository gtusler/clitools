use std::{fmt::Display, str::FromStr};

#[derive(Debug, Copy, Clone)]
pub enum BarStyle {
    HashesPlain,
    HashesDashes,
    ArrowThin,
    ArrowThick,
}

impl BarStyle {
    pub fn to_pattern(&self) -> String {
        match self {
            BarStyle::HashesPlain => String::from("[#  ]"),
            BarStyle::HashesDashes => String::from("(#--)"),
            BarStyle::ArrowThin => String::from("(->.)"),
            BarStyle::ArrowThick => String::from("(=>.)"),
        }
    }
}

impl FromStr for BarStyle {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match &*s.to_ascii_lowercase() {
            "hashes-plain" => BarStyle::HashesPlain,
            "hashes-dashes" => BarStyle::HashesDashes,
            "arrow-thin" => BarStyle::ArrowThin,
            "arrow-thick" => BarStyle::ArrowThick,
            _ => return Err(format!("unknown style: `{s}`")),
        })
    }
}

impl Display for BarStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_pattern())
    }
}
