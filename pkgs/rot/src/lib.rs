use clap::{Arg, ArgMatches, Command};
use librs::rot::{rot, Charset};

pub fn command() -> Command {
    Command::new("rot")
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
}

pub fn handle(matches: &ArgMatches) -> i32 {
    let rotation_string = matches.get_one::<String>("rotation").unwrap();
    let rotation: u8 = rotation_string.parse::<u8>().unwrap();

    let charset_default = String::from("a9");
    let charset_raw = matches
        .get_one::<String>("charset")
        .unwrap_or(&charset_default);
    let charset = Charset::from_string(charset_raw.to_string());

    if let Err(e) = charset {
        eprintln!("Error: {}", e);
        return 1;
    }


    let input = matches.get_one::<String>("input").unwrap();
    let output = rot(rotation, input.to_string(), charset.unwrap());

    // println!("input: {}", input);
    // println!("rotation: {}", rotation);

    match output {
        Ok(o) => {
            println!("{}", o);
            0
        }
        Err(e) => {
            println!("Error: {}", e);
            1
        }
    }
}
