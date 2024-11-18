use color_print::cformat;
use std::path::PathBuf;

/// Expects an absolute file path
pub fn highlight_file_name_string(path: &str) -> String {
    let file_path = PathBuf::from(path);
    let file_name = file_path.file_name().unwrap().to_str().unwrap();
    let file_base = match file_path.parent() {
        Some(p) => p.display().to_string(),
        None => String::new(),
    };

    cformat!(
        "{}{}<r>{}</>",
        file_base,
        std::path::MAIN_SEPARATOR_STR,
        file_name
    )
}
