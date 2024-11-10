use clap::Command;
use librs::cli::cli_style::cli_style;

pub fn command() -> Command {
    Command::new("clitinfo")
        .about("Prints information about the other packages in clitools")
        .version("0.1.0")
        .styles(cli_style())
}
