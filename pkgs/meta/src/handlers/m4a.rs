use librs::fs::filemeta::m4a::Mp4a;
use std::path::PathBuf;

pub fn handle(file_path: &PathBuf) -> i32 {
    println!("this is an m4a: {}", file_path.display());

    let m4a = match Mp4a::from_filepath(file_path) {
        Some(m) => m,
        None => {
            eprintln!("Failed to read file");
            return 1;
        }
    };

    m4a.print_full();

    0
}
