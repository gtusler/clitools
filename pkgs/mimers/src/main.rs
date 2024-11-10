fn main() {
    let matches = mimers::command().get_matches();
    let exit_code = mimers::handle(&matches);

    std::process::exit(exit_code);
}
