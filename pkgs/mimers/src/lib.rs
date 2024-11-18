use clap::{Arg, ArgMatches, Command};
use clap_complete::Shell;
use librs::cli::{
    cli_style::cli_style,
    gen_completion::{self, print_completions},
};
use mime_more::from_path_and_content;
use std::{fs::canonicalize, path::Path, process};

pub fn command() -> Command {
    Command::new("mimers")
        .about("Get mime-type information about the given file")
        .version("0.1.0")
        .styles(cli_style())
        .arg(
            Arg::new("file")
                .required(true)
                .help("The file to slap a mime type on"),
        )
        .arg(gen_completion::arg())
}

pub fn handle(matches: &ArgMatches) -> i32 {
    if let Some(generator) = matches.get_one::<Shell>("generator").copied() {
        let mut cmd = command();
        print_completions(generator, &mut cmd);
        process::exit(0);
    }

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
