use crate::commands;
use clap::{ArgMatches, Command};
use librs::cli::{cli_style::cli_style, gen_completion};

pub fn command() -> Command {
    Command::new("timers")
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
        .subcommand(commands::db::command())
        .subcommand(gen_completion::subcommand())
}

pub fn handle(matches: &ArgMatches) -> i32 {
    match matches.subcommand().expect("No subcommand provided") {
        ("start", start_matches) => commands::start::handle(start_matches),
        ("stop", stop_matches) => commands::stop::handle(stop_matches),
        ("abort", abort_matches) => commands::abort::handle(abort_matches),
        ("project", project_matches) => commands::project::handle(project_matches),
        ("db", db_matches) => commands::db::handle(db_matches),
        ("generate-completion", gen_matches) => commands::gen::handle(gen_matches),
        _ => unreachable!(),
    }
}
