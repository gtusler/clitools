use crate::db::create::create_db;
use clap::{ArgMatches, Command};

pub fn command() -> Command {
    Command::new("init")
        .about("Initialise the internal timer database in a globally accessible location")
}

pub fn handle(_matches: &ArgMatches) -> i32 {
    match create_db() {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("{}", e);
            1
        }
    }
}
