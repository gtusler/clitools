fn main() {
    let matches = sysinfo::command().get_matches();
    let exit_code = sysinfo::handle(&matches);

    std::process::exit(exit_code);
}
