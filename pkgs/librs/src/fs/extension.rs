#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Extension {
    Doc,
    Docx,
    Gif,
    Jpg,
    Js,
    Md,
    Mp3,
    M4a,
    Png,
    Rs,
    Ts,
    Txt,
    Unknown(String),
}

impl Extension {
    /// Extract an extension from a string.
    /// Splits on the last `.` character.
    /// Gives `None` if there is no `.`.
    pub fn extract(input: &str) -> Option<Extension> {
        if !input.contains(".") {
            return None;
        }

        let after_dot = match input.split(".").collect::<Vec<&str>>().last() {
            Some(a) => a.to_owned(),
            None => {
                return None;
            }
        };

        match after_dot {
            "doc" => Some(Extension::Doc),
            "docx" => Some(Extension::Docx),
            "gif" => Some(Extension::Gif),
            "jpg" => Some(Extension::Jpg),
            "js" => Some(Extension::Js),
            "md" => Some(Extension::Md),
            "mp3" => Some(Extension::Mp3),
            "m4a" => Some(Extension::M4a),
            "png" => Some(Extension::Png),
            "rs" => Some(Extension::Rs),
            "ts" => Some(Extension::Ts),
            "txt" => Some(Extension::Txt),
            _ => Some(Extension::Unknown(after_dot.to_owned())),
        }
    }
}

impl std::fmt::Display for Extension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::Doc => ".doc",
            Self::Docx => ".docx",
            Self::Gif => ".gif",
            Self::Jpg => ".jpg",
            Self::Js => ".js",
            Self::Md => ".md",
            Self::Mp3 => ".mp3",
            Self::M4a => ".m4a",
            Self::Png => ".png",
            Self::Rs => ".rs",
            Self::Ts => ".ts",
            Self::Txt => ".txt",
            Self::Unknown(ext) => ext,
        };

        write!(f, "{}", msg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod extraction {
        use super::*;

        #[test]
        fn no_extension() {
            assert_eq!(Extension::extract(""), None);
            assert_eq!(Extension::extract("nothing"), None);
        }

        #[test]
        fn extract_mp3() {
            assert_eq!(Extension::extract(".mp3"), Some(Extension::Mp3));
            assert_eq!(Extension::extract("something.mp3"), Some(Extension::Mp3));
        }

        #[test]
        fn extract_m4a() {
            assert_eq!(Extension::extract(".m4a"), Some(Extension::M4a));
            assert_eq!(Extension::extract("something.m4a"), Some(Extension::M4a));
        }

        #[test]
        fn longer() {
            assert_eq!(Extension::extract(".docx"), Some(Extension::Docx));
            assert_eq!(Extension::extract("something.docx"), Some(Extension::Docx));
        }

        #[test]
        fn shorter() {
            assert_eq!(Extension::extract("README.md"), Some(Extension::Md));
        }

        #[test]
        fn unknown() {
            assert_eq!(
                Extension::extract("something.xyz"),
                Some(Extension::Unknown("xyz".to_string()))
            );
        }
    }

    #[test]
    fn comparison() {
        assert_eq!(Extension::Docx, Extension::Docx);
        assert_ne!(Extension::Doc, Extension::Docx);

        assert!(Extension::Png == Extension::Png);
        assert!(Extension::Jpg != Extension::Png);
    }
}
