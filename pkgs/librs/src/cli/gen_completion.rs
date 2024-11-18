use clap::{value_parser, Arg, ArgAction, Command};
use clap_complete::{
    aot::{generate, Generator},
    Shell,
};
use std::io;

pub fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}

pub fn arg() -> Arg {
    Arg::new("generator")
        .long("generate-completion")
        .help("Generates completion scripts for the given command")
        .exclusive(true)
        .action(ArgAction::Set)
        .value_parser(value_parser!(Shell))
}

pub fn subcommand() -> Command {
    Command::new("generate-completion")
        .about("Generates completion scripts for the given command")
        .arg(
            Arg::new("shell")
                .required(true)
                .exclusive(true)
                .action(ArgAction::Set)
                .value_parser(value_parser!(Shell)),
        )
}
