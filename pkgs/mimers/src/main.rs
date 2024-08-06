use std::{fs::canonicalize, path::Path, process};

use clap::{Arg, Command};
use mime_more::from_path_and_content;

fn main() {
    let matches = Command::new("mimers")
        .about("Get mime-type information about the given file")
        .version("0.1.0")
        .arg(
            Arg::new("file")
                .required(true)
                .help("The file to slap a mime type on"),
        )
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

    // let file_extension = file_path_absolute.extension();

    let mime_result = from_path_and_content(&file_path_absolute);

    match mime_result {
        Ok(mime) => {
            println!("{}", mime);
        }
        Err(err) => {
            println!("{}", err);
            // println!("Unable to read mime type. ({}) {}", err.kind(), err.to_string());
            process::exit(1);
        }
    }
}
