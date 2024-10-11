use clap::{ArgMatches, Command};

mod list;
mod remove;
mod summary;

pub fn command() -> Command {
    Command::new("project")
        .about("Various tools for managing projects")
        .subcommand_required(true)
        .subcommand(summary::command())
        .subcommand(remove::command())
        .subcommand(list::command())
}

pub fn handle(matches: &ArgMatches) -> i32 {
    let exit_code = match matches.subcommand().expect("subcommand required") {
        ("list", list_matches) => list::handle(list_matches),
        ("summary", summary_matches) => summary::handle(summary_matches),
        ("remove", remove_matches) => remove::handle(remove_matches),
        _ => unreachable!(),
    };

    exit_code
}
