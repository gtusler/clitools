fn main() {
    let matches = uninstaller::command().get_matches();
    let exit_code = uninstaller::handle(&matches);

    std::process::exit(exit_code);
}
