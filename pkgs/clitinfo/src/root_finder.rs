use std::{path::PathBuf, process};

use dirs::home_dir;

pub fn find_root() -> Option<PathBuf> {
    let home = match home_dir() {
        None => {
            eprintln!("Unable to get home dir");
            process::exit(1);
        }
        Some(h) => h,
    };

    let home_string = home.to_str().unwrap().to_owned();

    let possible_paths: Vec<PathBuf> = vec![PathBuf::from(home_string + "/dev/rs/clitools")];

    possible_paths
        .into_iter()
        .find(|possible_path| possible_path.exists() && possible_path.is_dir())
}
