use clap::{builder::PossibleValuesParser, Arg, Command};
use clap_complete::Shell;
use installer::fs::main_dirs;
use librs::cli::{
    cli_style::cli_style,
    gen_completion::{self, print_completions},
};
use std::process;

fn command() -> Command {
    Command::new("clit-install")
        .about("Perform initial system-wide setup")
        .long_about(
            r#"Perform initial system-wide setup.

Executes the following steps:
1. Generate shell completion files for each bin, using clap_generate.
2. Move the binary files to a global location on the PATH.
3. Move shell completion scripts to a global location.
4. Optionally append shell completion loaders to the shell's rc file.

Ultimately, that's the plan. No part of it is working at the moment.
Thought it would make a nice feature.
            "#,
        )
        .styles(cli_style())
        .arg(
            Arg::new("shell")
                .help("Which shell to include completion scripts for")
                .value_parser(PossibleValuesParser::new(["bash", "zsh"])),
        )
        .arg(gen_completion::arg())
}

fn main() {
    let matches = command().get_matches();

    if let Some(generator) = matches.get_one::<Shell>("generator").copied() {
        let mut cmd = command();
        print_completions(generator, &mut cmd);
        process::exit(0);
    }

    let main_dir = match main_dirs::main_dir() {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };
    let exe_dir = match main_dirs::exe_dir() {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    println!("main_dir: {}", main_dir.display());
    println!("exe_dir: {}", exe_dir.display());

    // println!("installer unimplemented");
}
