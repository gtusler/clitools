fn main() {
    let matches = listdirs::command().get_matches();
    let exit_code = listdirs::handle(&matches);

    std::process::exit(exit_code);
}
