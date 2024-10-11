use std::process;

use clap::Command;
use librs::cli::cli_style::cli_style;
use timers::commands;

fn main() {
    let command = Command::new("timers")
        .about("Project time tracker in rust")
        .long_about(
            r#"Project time tracker in rust.

There can only be one running tracker at a time.
"#,
        )
        .styles(cli_style())
        .color(clap::ColorChoice::Auto)
        .subcommand_required(true)
        .subcommand(commands::start::command())
        .subcommand(commands::stop::command())
        .subcommand(commands::abort::command())
        .subcommand(commands::project::command())
        .subcommand(commands::db::command());

    let matches = command.get_matches();

    let exit_code = match matches.subcommand().expect("No subcommand provided") {
        ("start", start_matches) => commands::start::handle(start_matches),
        ("stop", stop_matches) => commands::stop::handle(stop_matches),
        ("abort", abort_matches) => commands::abort::handle(abort_matches),
        ("project", project_matches) => commands::project::handle(project_matches),
        ("db", db_matches) => commands::db::handle(db_matches),
        _ => unreachable!(),
    };

    process::exit(exit_code);
}
