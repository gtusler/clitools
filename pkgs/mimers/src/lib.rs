use std::{fs::canonicalize, path::Path, process};
use mime_more::from_path_and_content;
use clap::{Arg, ArgMatches, Command};

pub fn command() -> Command {
    Command::new("mimers")
        .about("Get mime-type information about the given file")
        .version("0.1.0")
        .arg(
            Arg::new("file")
                .required(true)
                .help("The file to slap a mime type on"),
        )
}

pub fn handle(matches: &ArgMatches) -> i32 {
    let file_path_str = matches.get_one::<String>("file").expect("required arg");
    let file_path = Path::new(file_path_str);

    if !file_path.exists() {
        eprintln!("File path `{}` doesn't exist", file_path_str);
        return 1;
    }

    let file_path_absolute = canonicalize(file_path).unwrap();

    if !file_path_absolute.is_file() {
        println!(
            "Path `{}` doesn't point to a file",
            file_path_absolute.display()
        );
        process::exit(1);
    }

    match from_path_and_content(&file_path_absolute) {
        Ok(mime) => {
            println!("{}", mime);
            0
        }
        Err(err) => {
            println!("{}", err);
            // println!("Unable to read mime type. ({}) {}", err.kind(), err.to_string());
            1
        }
    }
}
