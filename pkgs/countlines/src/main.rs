use std::{
    fs::{self, canonicalize},
    path::Path,
    process,
};

use clap::{Arg, Command};

fn main() {
    let matches = Command::new("countlines")
        .about("Counts the lines in the given file")
        .version("0.1.0")
        .arg(Arg::new("file").required(true).help("The file to count"))
        .get_matches();

    let file_path_str = matches.get_one::<String>("file").unwrap();
    let file_path = Path::new(file_path_str);

    if !file_path.exists() {
        println!("File path `{}` doesn't exist", file_path_str);
        process::exit(1);
    }

    let file_path_absolute = canonicalize(file_path).unwrap();

    if !file_path_absolute.is_file() {
        println!(
            "Path `{}` doesn't point to a file",
            file_path_absolute.display()
        );
        process::exit(1);
    }

    let file_contents = fs::read_to_string(file_path_absolute);

    match file_contents {
        Ok(contents) => {
            let file_contents_split = contents.split('\n').collect::<Vec<&str>>();
            println!("{:?}", file_contents_split.len());
        }
        Err(err) => {
            println!("Unable to read file. ({}) {}", err.kind(), err.to_string());
            process::exit(1);
        }
    }
}
