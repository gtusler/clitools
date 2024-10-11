use clap::{ArgMatches, Command};

pub fn command() -> Command {
    Command::new("empty").about("Completely empty the database, losing all data.")
}

pub fn handle(_matches: &ArgMatches) -> i32 {
    todo!()
}
