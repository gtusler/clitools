use clap::Command;
use librs::cli::cli_style::cli_style;

fn main() {
    let command = Command::new("clit-uninstall")
        .about("Perform system-wide removal of clitools")
        .styles(cli_style());

    let _matches = command.get_matches();

    println!("uninstalled successfully");
}
