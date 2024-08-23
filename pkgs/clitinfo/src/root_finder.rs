use std::{path::PathBuf, process};

use dirs::home_dir;

pub fn find_root() -> Option<PathBuf> {
    let home = home_dir();

    if let None = home {
        eprintln!("Unable to get home dir");
        process::exit(1);
    }

    let home = home.unwrap();
    let home_str = home.to_str().unwrap();

    // println!("home: {:?}", home);

    let possible_paths: Vec<PathBuf> =
        vec![PathBuf::from(home_str.to_owned() + "/dev/rs/clitools")];

    for possible_path in possible_paths {
        if possible_path.exists() && possible_path.is_dir() {
            // println!("path {} exists", possible_path.to_str().unwrap());
            return Some(possible_path);
        }
    }

    None
}
