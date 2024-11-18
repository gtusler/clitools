use crate::commands;
use clap::{ArgMatches, Command};
use librs::cli::{cli_style::cli_style, gen_completion};

pub fn command() -> Command {
    Command::new("progress")
        .about("An exploration of termnial progress bars")
        .version("0.1.0")
        .styles(cli_style())
        .subcommand_required(true)
        .subcommand(commands::progress::command())
        .subcommand(commands::ml_progress::command())
        .subcommand(commands::progressing::command())
        .subcommand(commands::govi::command())
        .subcommand(gen_completion::subcommand())
        .arg(gen_completion::arg())
}

pub fn handle(matches: &ArgMatches) -> i32 {
    match matches.subcommand().expect("subcommand required") {
        ("progress", progress_matches) => commands::progress::handle(progress_matches),
        ("ml-progress", ml_progress_matches) => commands::ml_progress::handle(ml_progress_matches),
        ("progressing", progressing_matches) => commands::progressing::handle(progressing_matches),
        ("govi", govi_matches) => commands::govi::handle(govi_matches),
        ("generate-completion", gen_matches) => commands::gen::handle(gen_matches),
        _ => unreachable!(),
    }
}
