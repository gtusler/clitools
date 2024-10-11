use clap::{ArgMatches, Command};
use color_print::cprintln;
use librs::datetime::datetime_formatter::DateTimeFormatter;

use crate::db::{entity::timer::Timer, main_db::connect_main_db};

pub fn command() -> Command {
    Command::new("stop").about("Stop the running timer, adding to the total time for the project")
}

pub fn handle(_matches: &ArgMatches) -> i32 {
    let connection = match connect_main_db() {
        Ok(con) => con,
        Err(e) => {
            eprintln!("{}", e);
            return 1;
        }
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

    let time_now = chrono::Utc::now();
    let time_str = DateTimeFormatter::ymd_hms(&time_now);

    match Timer::stop(&connection, &time_str) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{}", e);
            return 1;
        }
    }

    cprintln!("Timer stopped at {}", DateTimeFormatter::hms(&time_now));

    0
}
