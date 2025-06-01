use clap::{builder::PathBufValueParser, Arg, ArgAction, ArgMatches, Command, ValueHint};
use clap_complete::Shell;
use librs::{
    cli::{
        cli_style::cli_style,
        gen_completion::{self, print_completions},
    },
    fs::{dir_tree::tree::Tree, extension::Extension},
};
use std::path::PathBuf;

pub fn command() -> Command {
    Command::new("treers")
        .about("Print a directory as a tree")
        .styles(cli_style())
        .arg(gen_completion::arg())
        .arg(
            Arg::new("dir")
                .help("The directory to tree")
                .required(true)
                .value_hint(ValueHint::DirPath)
                .value_parser(PathBufValueParser::new()),
        )
        .arg(
            Arg::new("dirs-only")
                .long("dirs-only")
                .short('d')
                .action(ArgAction::SetTrue)
                .help("Only show directories in the output.")
        )
        .arg(
            Arg::new("extension")
                .long("extension")
                .short('e')
                .action(ArgAction::Append)
                .help("A file extension to filter the tree. Must include the `.` prefix. Can be set multiple times.")
        )
}

pub fn handle(matches: &ArgMatches) -> i32 {
    if let Some(generator) = matches.get_one::<Shell>("generator").copied() {
        let mut cmd = command();
        print_completions(generator, &mut cmd);
        return 0;
    }

    let dir = matches.get_one::<PathBuf>("dir").expect("arg required");
    let dirs_only = matches
        .get_one::<bool>("dirs-only")
        .expect("defaults to false");

    let extensions_raw = matches.get_many::<String>("extension");

    let mut tree = match Tree::from_path(dir, true) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{}", e);
            return 1;
        }
    };

    if *dirs_only {
        tree.dirs_only();
    }

    if let Some(extensions_r) = extensions_raw {
        let extensions = extensions_r
            .filter_map(|ext| Extension::extract(ext))
            .collect::<Vec<Extension>>();

        tree.with_extensions(&extensions);
    }

    println!("{}", tree);

    0
}
