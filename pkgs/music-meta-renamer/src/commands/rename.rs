use clap::{builder::PathBufValueParser, Arg, ArgAction, ArgGroup, ArgMatches, Command, ValueHint};
use librs::fs::{
    dir_tree::{leaf::Leaf, tree::Tree},
    extension::Extension,
    filemeta::m4a::Mp4a,
};
use regex::Regex;
use std::{collections::HashMap, path::{Path, PathBuf}};

pub fn command() -> Command {
    Command::new("rename")
        .about("Rename files")
        .arg(
            Arg::new("recursive")
                .help("Finds all files in the current directory and sub-directories.")
                .long("recursive")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("flatten")
                .help("Flatten all the files into a single directory.")
                .long("flatten")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("outdir")
                .help("Set the directory where the renamed files will be placed. The dir will be created if it doesn't exist. No overwriting files takes place.")
                .long("outdir")
                .value_hint(ValueHint::DirPath)
                .value_parser(PathBufValueParser::new())
        )
        .arg(
            Arg::new("format")
                .help("Set the format for the new file names. There should be some parsing here, I need to figure out how this works.")
                .long("format")
                .short('f')
                .default_value("%artist% - %song%")
        )
        .arg(
            Arg::new("rename-in-place")
                .help("Rename the files where they are. No copying takes place.")
                .long("rename-in-place")
                .action(ArgAction::SetTrue)
                .conflicts_with("outdir")
        )
        .arg(
            Arg::new("input")
                .help("The path to either a file or folder.")
                .required(true)
                .value_hint(ValueHint::AnyPath)
                .value_parser(PathBufValueParser::new())
        )
        .groups(vec![
            ArgGroup::new("input"),
            ArgGroup::new("output"),
            ArgGroup::new("formatting"),
        ])
}

pub fn handle(matches: &ArgMatches) -> i32 {
    let recursive = matches.get_one::<bool>("recursive").expect("has default");
    let _flatten = matches.get_one::<bool>("flatten").expect("has default");
    let outdir_raw = matches.get_one::<PathBuf>("outdir");
    let format_raw = matches.get_one::<String>("format").expect("has default");
    let format_regex = Regex::new(r"%([^%]+)%").unwrap();
    let token_names = format_regex
        .captures_iter(format_raw)
        .filter_map(|caps| caps.get(1).map(|m| m.as_str()))
        .collect::<Vec<&str>>();
    let rename_in_place = matches
        .get_one::<bool>("rename-in-place")
        .expect("has default");
    let input = matches.get_one::<PathBuf>("input").expect("is required");

    if !input.exists() {
        eprintln!("File {} doesn't exist", input.display());
        return 1;
    }

    if input.is_symlink() {
        eprintln!("Symbolic links are not supported");
        return 1;
    }

    let outdir = if *rename_in_place {
        match input.parent() {
            Some(path) => path,
            None => {
                eprintln!("Can't determine input path parent dir. Maybe I should canonicalize the path first.");
                return 1;
            }
        }
    } else {
        match outdir_raw {
            Some(path) => path,
            None => {
                eprintln!("If we're not renaming in place, --outdir must be set.");
                return 1;
            }
        }
    };

    if !outdir.exists() {
        match std::fs::create_dir_all(outdir) {
            Ok(()) => (),
            Err(e) => {
                eprintln!("Failed to create dir: {}", e);
                return 1;
            }
        }
    }

    if input.is_file() {
        handle_single_file(input, format_raw, &token_names, *rename_in_place, outdir);
        return 0;
    }

    if input.is_dir() {
        let mut tree = match Tree::from_path(input, *recursive) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("{}", e);
                return 1;
            }
        };

        tree.with_extensions(&vec![Extension::M4a]);

        for leaf in tree.leaves().iter() {
            handle_leaf(leaf, format_raw, &token_names, *rename_in_place, outdir);
        }

        return 0;
    }

    eprintln!("What is this?");
    1
}

fn handle_leaf(
    leaf: &Leaf,
    format_raw: &str,
    token_names: &[&str],
    rename_in_place: bool,
    outdir: &Path,
) {
    match leaf {
        Leaf::File { .. } => {
            handle_single_file(&leaf.full_path(), format_raw, token_names, rename_in_place, outdir);
        }
        Leaf::Dir { children, .. } => {
            for child_leaf in children {
                handle_leaf(child_leaf, format_raw, token_names, rename_in_place, outdir);
            }
        }
    }
}

fn handle_single_file(
    path: &PathBuf,
    format_raw: &str,
    token_names: &[&str],
    rename_in_place: bool,
    outdir: &Path,
) {
    let meta = match Mp4a::from_filepath(path) {
        Some(m) => m,
        None => panic!("Unable to get file metadata"),
    };
    let meta_hm = meta.into_hashmap();
    let new_file_name = replace_format_tokens(format_raw, token_names, &meta_hm);

    if rename_in_place {
        let output_path = path.with_file_name(&new_file_name);
        match std::fs::rename(path, &output_path) {
            Ok(()) => {
                println!("{} -> {}", path.display(), output_path.display());
            }
            Err(e) => {
                eprintln!("Failed to write file. {}", e);
            }
        }
    } else {
        let output_path = create_output_path(&new_file_name, outdir);
        match std::fs::copy(path, &output_path) {
            Ok(_size) => {
                println!("{} -> {}", path.display(), output_path.display());
            }
            Err(e) => {
                eprintln!("Failed to write file. {}", e);
            }
        }
    }
}

/// Replaces tokens in `format_raw` with with values from the `values` hashmap.
/// Uses the given `format_caps` to figure out what to look up in `values`.
/// If `format_caps` is `None`, returns `format_raw`.
fn replace_format_tokens(
    format_raw: &str,
    token_names: &[&str],
    values: &HashMap<&str, String>,
) -> String {
    let mut result = format_raw.to_owned();

    for name in token_names {
        if let Some(value) = values.get(name) {
            let token = format!("%{}%", name);
            result = result.replace(&token, value);
        }
    }

    result
}

fn create_output_path(filename: &str, outdir: &Path) -> PathBuf {
    let mut output = outdir.to_owned();
    output.push(filename);
    output
}
