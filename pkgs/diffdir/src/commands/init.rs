use clap::{builder::PathBufValueParser, Arg, ArgAction, ArgMatches, Command, ValueHint};
use clap_complete::Shell;
use librs::{
    cli::{
        cli_style::cli_style,
        gen_completion::{self, print_completions},
    },
    fs::{
        dir_tree::Tree, dir_tree_comparison::TreeComparison,
        dir_tree_similarity::print_similarities,
    },
};
use std::path::PathBuf;

pub fn command() -> Command {
    Command::new("diffdir")
        .about("Find the differences between the contents of two directories")
        .styles(cli_style())
        .arg(gen_completion::arg())
        .arg(
            Arg::new("dir1")
                .required(true)
                .value_hint(ValueHint::DirPath)
                .value_parser(PathBufValueParser::new())
        )
        .arg(
            Arg::new("dir2")
                .required(true)
                .value_hint(ValueHint::DirPath)
                .value_parser(PathBufValueParser::new())
        )
        .arg(
            Arg::new("recursive")
                .help("Whether or not to recursively scan both directories")
                .long("recursive")
                .short('r')
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("use-meta")
                .help("Whether or not to use metadata (like file size, modified dates etc) to match file system entries")
                .long("use-meta")
                .short('m')
                .action(ArgAction::SetTrue)
        )
}

pub fn handle(matches: &ArgMatches) -> i32 {
    if let Some(generator) = matches.get_one::<Shell>("generator").copied() {
        let mut cmd = command();
        print_completions(generator, &mut cmd);
        return 0;
    }

    let dir1 = matches.get_one::<PathBuf>("dir1").expect("arg required");
    let dir2 = matches.get_one::<PathBuf>("dir2").expect("arg required");

    if !dir1.exists() {
        eprintln!("dir1 doesn't exist: {}", dir1.display());
        return 1;
    }

    if !dir2.exists() {
        eprintln!("dir2 doesn't exist: {}", dir2.display());
        return 1;
    }

    if !dir1.is_dir() {
        eprintln!("dir1 is not a directory: {}", dir1.display());
        return 1;
    }

    if !dir2.is_dir() {
        eprintln!("dir2 is not a directory: {}", dir2.display());
        return 1;
    }

    let recursive = matches.get_one::<bool>("recursive").expect("always set");
    let _use_meta = matches.get_one::<bool>("use-meta").expect("always set");

    let dir1_tree = match Tree::from_path(dir1, *recursive) {
        Ok(tree) => tree,
        Err(e) => {
            eprintln!("{}", e);
            return 1;
        }
    };
    let dir2_tree = match Tree::from_path(dir2, *recursive) {
        Ok(tree) => tree,
        Err(e) => {
            eprintln!("{}", e);
            return 1;
        }
    };

    let tree_comparison = TreeComparison::new(&dir1_tree, &dir2_tree);

    let similarities = tree_comparison.find_similarities();
    print_similarities(&similarities);

    // let differences = tree_comparison.find_differences();
    // print_differences(&differences);

    0
}
