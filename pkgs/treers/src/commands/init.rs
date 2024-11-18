use std::path::PathBuf;

use clap::{builder::PathBufValueParser, Arg, ArgMatches, Command, ValueHint};
use clap_complete::Shell;
use librs::{cli::gen_completion::{self, print_completions}, fs::dir_tree::Tree};

pub fn command() -> Command {
    Command::new("treers")
        .about("Print a directory as a tree")
        .arg(gen_completion::arg())
        .arg(
            Arg::new("dir")
                .help("The directory to tree")
                .required(true)
                .value_hint(ValueHint::DirPath)
                .value_parser(PathBufValueParser::new())
        )
}

pub fn handle(matches: &ArgMatches) -> i32 {
    if let Some(generator) = matches.get_one::<Shell>("generator").copied() {
        let mut cmd = command();
        print_completions(generator, &mut cmd);
        return 0;
    }

    let dir = matches.get_one::<PathBuf>("dir").expect("arg required");

    let tree = match Tree::from_path(dir, true) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{}", e);
            return 1;
        }
    };

    println!("{}", tree);

    0
}
