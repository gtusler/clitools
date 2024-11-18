use std::process;
use treers::commands;

fn main() {
    let matches = commands::init::command().get_matches();
    let exit_code = commands::init::handle(&matches);

    process::exit(exit_code);
}
