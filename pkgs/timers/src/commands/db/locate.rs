use clap::{ArgMatches, Command};

use crate::db::main_db::main_db_path;

pub fn command() -> Command {
    Command::new("locate").about("Print the location of the global database")
}

pub fn handle(_matches: &ArgMatches) -> i32 {
    match main_db_path() {
        Ok(d) => {
            println!("{}", d.display());
            0
        }
        Err(e) => {
            eprintln!("{}", e);
            1
        }
    }
}
