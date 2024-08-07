use std::process;

use clap::{Arg, Command};
use librs::rot::{rot, Charset};

fn main() {
    let matches = Command::new("rot")
        .about("The rot cipher in cli form")
        .arg(
            Arg::new("rotation")
                .required(true)
                .help("How much of a rotation should be used"),
        )
        .arg(
            Arg::new("charset")
                .help("One of: (az, a9)")
                .short('c')
                .long("charset"),
        )
        .arg(Arg::new("input").required(true).help("A string to rotate"))
        .get_matches();

    let rotation_string = matches.get_one::<String>("rotation").unwrap();
    let rotation: u8 = rotation_string.parse::<u8>().unwrap();

    let charset_default = String::from("a9");
    let charset_raw = matches
        .get_one::<String>("charset")
        .unwrap_or(&charset_default);
    let charset = Charset::from_string(charset_raw.to_string());

    match charset {
        Err(e) => {
            println!("Error: {}", e);
            process::exit(1);
        }
        _ => (),
    }

    let input = matches.get_one::<String>("input").unwrap();
    let output = rot(rotation, input.to_string(), charset.unwrap());

    // println!("input: {}", input);
    // println!("rotation: {}", rotation);

    match output {
        Ok(o) => {
            println!("{}", o);
        }
        Err(e) => {
            println!("Error: {}", e);
            process::exit(1);
        }
    }
}
