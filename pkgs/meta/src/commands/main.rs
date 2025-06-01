use crate::handlers::m4a;
use clap::{builder::PathBufValueParser, Arg, ArgMatches, Command, ValueHint};
use librs::{cli::cli_style::cli_style, fs::extension::Extension};
use std::path::PathBuf;

pub fn command() -> Command {
    Command::new("meta")
        .styles(cli_style())
        .about("Extract metadata from various file types")
        .arg(
            Arg::new("file")
                .help("A file path to get metadata from.")
                .value_hint(ValueHint::FilePath)
                .value_parser(PathBufValueParser::new())
                .required(true),
        )
}

pub fn handle(matches: &ArgMatches) -> i32 {
    let file_path = matches.get_one::<PathBuf>("file").expect("required arg");
    let file_path_str = format!("{}", file_path.display());

    let extension = match Extension::extract(&file_path_str) {
        Some(ext) => ext,
        None => {
            eprintln!("Unknown file extension");
            return 1;
        }
    };

    if !file_path.exists() {
        eprintln!("File path doesn't exist.");
        return 1;
    }

    if !file_path.is_file() {
        eprintln!("The given path doesn't point to a file.");
        return 1;
    }

    match extension {
        Extension::M4a => m4a::handle(file_path),
        _ => {
            println!("Unsupported file extension");

            1
        }
    }
}
