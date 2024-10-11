use crate::db::{entity::timer::Timer, main_db::connect_main_db};
use clap::{ArgMatches, Command};

pub fn command() -> Command {
    Command::new("abort")
        .about("Abort the running timer, not adding to the total time for the project")
}

pub fn handle(_matches: &ArgMatches) -> i32 {
    let connection = match connect_main_db() {
        Err(e) => {
            eprintln!("{}", e);
            return 1;
        }
        Ok(con) => con,
    };

    match Timer::is_running(&connection) {
        Ok(true) => (),
        Ok(false) => {
            eprintln!("There is no running timer.");
            return 1;
        }
        Err(e) => {
            eprintln!("{}", e);
            return 1;
        }
    }

    if let Err(e) = Timer::abort(&connection) {
        eprintln!("{}", e);
        return 1;
    }

    0
}
