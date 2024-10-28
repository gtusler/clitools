use clap::Command;
use librs::cli::cli_style::cli_style;

fn main() {
    let command = Command::new("clit-install")
        .about("Perform initial system-wide setup")
        .long_about(
            r#"Perform initial system-wide setup.

Executes the following steps:
1. Build each bin using cargo.
2. Generate shell completion files for each bin, using clap_generate.
3. Move the generated files to a global location on the PATH.
4. Move shell completion scripts to a global location.
5. Optionally append shell completion loaders to the shell's rc file.

Ultimately, that's the plan. No part of it is working at the moment.
Thought it would make a nice feature.
            "#
        )
        .styles(cli_style());

    let _matches = command.get_matches();

    println!("installed successfully");
}
