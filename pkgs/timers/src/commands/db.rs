use clap::{ArgMatches, Command};

mod empty;
mod init;
mod locate;

pub fn command() -> Command {
    Command::new("db")
        .about("High level control of the timer database")
        .subcommand_required(true)
        .subcommand(init::command())
        .subcommand(empty::command())
        .subcommand(locate::command())
}

pub fn handle(matches: &ArgMatches) -> i32 {
    let exit_code = match matches.subcommand().expect("subcommand required") {
        ("init", init_matches) => init::handle(init_matches),
        ("empty", empty_matches) => empty::handle(empty_matches),
        ("locate", locate_matches) => locate::handle(locate_matches),
        _ => unreachable!(),
    };

    exit_code
}
