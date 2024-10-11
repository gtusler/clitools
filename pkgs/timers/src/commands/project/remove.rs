use clap::{Arg, ArgMatches, Command};

use crate::db::main_db::connect_main_db;

pub fn command() -> Command {
    Command::new("remove")
        .about("Remove all traces of a project")
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

    let projects_query = format!("DELETE FROM projects WHERE name=\"{}\"", project_name);

    match connection.execute(projects_query) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{}", e);
            return 1;
        }
    }

    let timings_query = format!("DELETE FROM timings WHERE project=\"{}\"", project_name);

    match connection.execute(timings_query) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{}", e);
            return 1;
        }
    }

    let running_query = format!("DELETE FROM running WHERE project=\"{}\"", project_name);

    match connection.execute(running_query) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{}", e);
            return 1;
        }
    }

    0
}
