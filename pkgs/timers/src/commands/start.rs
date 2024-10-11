use clap::{Arg, ArgMatches, Command};
use color_print::cprintln;
use librs::datetime::datetime_formatter::DateTimeFormatter;

use crate::db::{
    entity::{project::Project, timer::Timer},
    main_db::connect_main_db,
};

pub fn command() -> Command {
    Command::new("start")
        .about("Start a new timer")
        .arg(Arg::new("project").required(true))
}

pub fn handle(matches: &ArgMatches) -> i32 {
    let connection = match connect_main_db() {
        Err(e) => {
            eprintln!("{}", e);
            return 1;
        }
        Ok(con) => con,
    };

    // check for running timers, bail if there is one.
    match Timer::is_running(&connection) {
        Ok(false) => (),
        Ok(true) => {
            eprintln!(
                "There is a timer running. Either stop or abort that before starting another one."
            );
            return 1;
        }
        Err(e) => {
            eprintln!("Timer Error: {}", e);
            return 1;
        }
    }

    let project_name = matches.get_one::<String>("project").expect("Arg required");

    if !Project::exists(&connection, project_name) {
        match Project::add(&connection, project_name) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{}", e);
                return 1;
            }
        }

        println!("Added project \"{}\"", project_name);
    }

    let time_now = chrono::Utc::now();
    let time_str = DateTimeFormatter::ymd_hms(&time_now);

    match Timer::start(&connection, project_name, &time_str) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{}", e);
            return 1;
        }
    }

    cprintln!(
        "{} timer started at {}",
        project_name,
        DateTimeFormatter::hms(&time_now)
    );

    0
}
