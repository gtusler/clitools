fn main() {
    let matches = countlines::command().get_matches();
    let exit_code = countlines::handle(&matches);

    std::process::exit(exit_code);
}
