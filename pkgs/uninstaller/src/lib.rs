use clap::{ArgMatches, Command};
use clap_complete::Shell;
use librs::cli::{
    cli_style::cli_style,
    gen_completion::{self, print_completions},
};
use std::process;

pub fn command() -> Command {
    Command::new("clit-uninstall")
        .about("Perform system-wide removal of clitools")
        .styles(cli_style())
        .arg(gen_completion::arg())
}

pub fn handle(matches: &ArgMatches) -> i32 {
    if let Some(generator) = matches.get_one::<Shell>("generator").copied() {
        let mut cmd = command();
        print_completions(generator, &mut cmd);
        process::exit(0);
    }

    println!("unimplemented");

    0
}
