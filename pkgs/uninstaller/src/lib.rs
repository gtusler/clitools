use clap::{ArgMatches, Command};
use librs::cli::cli_style::cli_style;

pub fn command() -> Command {
    Command::new("clit-uninstall")
        .about("Perform system-wide removal of clitools")
        .styles(cli_style())
}

pub fn handle(_matches: &ArgMatches) -> i32 {
    println!("unimplemented");

    0
}
