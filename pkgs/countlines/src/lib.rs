use std::{
    fs::{self, canonicalize},
    path::Path,
};
use clap::{Arg, ArgMatches, Command};

pub fn command() -> Command {
    Command::new("countlines")
        .about("Counts the lines in the given file")
        .version("0.1.0")
        .arg(Arg::new("file").required(true).help("The file to count"))
}

pub fn handle(matches: &ArgMatches) -> i32 {
    let file_path_str = matches.get_one::<String>("file").unwrap();
    let file_path = Path::new(file_path_str);

    if !file_path.exists() {
        eprintln!("File path `{}` doesn't exist", file_path_str);
        return 1;
    }

    let file_path_absolute = canonicalize(file_path).unwrap();

    if !file_path_absolute.is_file() {
        eprintln!(
            "Path `{}` doesn't point to a file",
            file_path_absolute.display()
        );
        return 1;
    }

    match fs::read_to_string(file_path_absolute) {
        Ok(contents) => {
            let file_contents_split = contents.split('\n').collect::<Vec<&str>>();
            println!("{:?}", file_contents_split.len());
            0
        }
        Err(err) => {
            eprintln!("Unable to read file. ({}) {}", err.kind(), err);
            1
        }
    }
}
