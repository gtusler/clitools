use clap::{Arg, ArgMatches, Command};
use color_print::cprintln;

use crate::{
    db::{
        entity::{
            project::{Project, ProjectError},
            timing::Timing,
        },
        main_db::connect_main_db,
    },
    view::timings::print_timings,
};

pub fn command() -> Command {
    Command::new("summary")
        .about("Break down the individual timers for a project.")
        .arg(Arg::new("project").required(true))
}

pub fn handle(matches: &ArgMatches) -> i32 {
    let connection = match connect_main_db() {
        Ok(con) => con,
        Err(e) => {
            eprintln!("{}", e);
            return 1;
        }
    };

    let project_name = matches.get_one::<String>("project").expect("required arg");

    match Project::fetch_running(&connection) {
        Ok(proj) => {
            if proj.name == *project_name {
                cprintln!("<g>{}</> is currently running", project_name);
            }
        }
        Err(ProjectError::NotRunning) => (),
        Err(e) => {
            eprintln!("{}", e);
            return 1;
        }
    }

    let timings = match Timing::fetch_project_all(&connection, project_name) {
        Ok(ts) => ts,
        Err(e) => {
            eprintln!("{}", e);
            return 1;
        }
    };

    print_timings(timings);

    0
}
