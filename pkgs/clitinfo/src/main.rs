use clap_complete::Shell;
use librs::{cli::gen_completion::print_completions, config_file::cargo_toml::CargoToml};
use root_finder::find_root;
use std::{fs::read_dir, process};

mod root_finder;

fn main() {
    let matches = clitinfo::command().get_matches();

    if let Some(generator) = matches.get_one::<Shell>("generator").copied() {
        let mut cmd = clitinfo::command();
        print_completions(generator, &mut cmd);
        process::exit(0);
    }

    println!("development of this has halted because it's a pain to parse Cargo.toml files. Tried stealing something from cargo source code but couldn't find anything immediately obvious");

    let root_dir = match find_root() {
        None => {
            eprintln!("Unable to find root directory.");
            process::exit(1);
        }
        Some(root) => root,
    };

    let mut pkgs_dir = root_dir.clone();
    pkgs_dir.push("pkgs");

    let project_entries = read_dir(pkgs_dir);

    if let Err(e) = project_entries {
        eprintln!("Failed to read root dir {}", e);
        process::exit(1);
    }

    let project_entries = project_entries.unwrap();

    for entry_result in project_entries {
        if let Err(e) = entry_result {
            eprintln!("{}", e);
            process::exit(1);
        }

        let entry = entry_result.unwrap();

        let entry_path = entry.path();
        let entry_name = entry_path.file_name().unwrap();

        let mut cargo_toml_path = entry_path.clone();
        cargo_toml_path.push("Cargo.toml");
        let cargo_toml = CargoToml::from_dir(&cargo_toml_path);
        if let Err(s) = cargo_toml {
            eprintln!("Cargo.toml: {}", s);
            process::exit(1);
        }
        let cargo_toml = cargo_toml.unwrap();

        println!("{:?}", entry_name);
        println!("{:?}", cargo_toml_path);
        println!("{}", cargo_toml);
        println!();
    }
}
