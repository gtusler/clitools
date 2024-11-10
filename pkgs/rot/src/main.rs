fn main() {
    let matches = rot::command().get_matches();
    let exit_code = rot::handle(&matches);

    std::process::exit(exit_code);
}
