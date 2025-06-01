use super::explain_format;
use crate::commands::rename;
use clap::{ArgMatches, Command};
use librs::cli::cli_style::cli_style;

pub fn command() -> Command {
    Command::new("music-meta-renamer")
        .about("A toolbox for renaming music files based on their metadata")
        .subcommand_required(true)
        .subcommand(rename::command())
        .subcommand(explain_format::command())
        .styles(cli_style())
}

pub fn handle(matches: &ArgMatches) -> i32 {
    match matches.subcommand().expect("subcommand required") {
        ("rename", rename_matches) => rename::handle(rename_matches),
        ("explain-format", explain_matches) => explain_format::handle(explain_matches),
        _ => unreachable!(),
    }
}
