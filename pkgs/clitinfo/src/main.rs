use clap::Command;
use librs::from_librs;

fn main() {
    let _ = Command::new("clitinfo")
        .about("Prints information about the other packages in clitools")
        .version("0.1.0")
        .get_matches();

    println!("not implemented");
    println!("{}", from_librs());
}
