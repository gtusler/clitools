use clap::{ArgMatches, Command};
use color_print::cprintln;

use crate::db::{entity::project::Project, main_db::connect_main_db};

pub fn command() -> Command {
    Command::new("list").about("List all the stored timers")
}

pub fn handle(_matches: &ArgMatches) -> i32 {
    let connection = match connect_main_db() {
        Err(e) => {
            eprintln!("{}", e);
            return 1;
        }
        Ok(con) => con,
    };

    let projects = match Project::fetch_all(&connection) {
        Err(e) => {
            eprintln!("{}", e);
            return 1;
        }
        Ok(ps) => ps,
    };

    cprintln!("There are <yellow>{}</> projects", projects.len());

    for project in projects {
        cprintln!("    <green>{}</>", project.name);
    }

    0
}
