use clap::Command;

pub fn command() -> Command {
    Command::new("clitinfo")
        .about("Prints information about the other packages in clitools")
        .version("0.1.0")
}
