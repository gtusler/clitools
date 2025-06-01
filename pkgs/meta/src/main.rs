use meta::commands;
use std::process;

fn main() {
    let matches = commands::main::command().get_matches();
    let exit_code = commands::main::handle(&matches);

    process::exit(exit_code);
}
