use std::path::PathBuf;

use clap::Command;
use librs::cli::cli_style::cli_style;

fn main() {
    let command = Command::new("listdirs")
        .about("List system specific directories")
        .styles(cli_style());

    let _matches = command.get_matches();

    println!("Home:     {}", string_if_some(dirs::home_dir()));
    println!("Desktop:  {}", string_if_some(dirs::desktop_dir()));
    println!("Document: {}", string_if_some(dirs::document_dir()));
    println!("Download: {}", string_if_some(dirs::download_dir()));
    println!("Video:    {}", string_if_some(dirs::video_dir()));
    println!("Audio:    {}", string_if_some(dirs::audio_dir()));
    println!("Picture:  {}", string_if_some(dirs::picture_dir()));
    println!("Public:   {}", string_if_some(dirs::public_dir()));
    println!("");
    println!("Preference:   {}", string_if_some(dirs::preference_dir()));
    println!("Config:       {}", string_if_some(dirs::config_dir()));
    println!("Config Local: {}", string_if_some(dirs::config_local_dir()));
    println!("");
    println!("Runtime:  {}", string_if_some(dirs::runtime_dir()));
    println!("Template: {}", string_if_some(dirs::template_dir()));
    println!("");
    println!("Font:       {}", string_if_some(dirs::font_dir()));
    println!("Cache:      {}", string_if_some(dirs::cache_dir()));
    println!("State:      {}", string_if_some(dirs::state_dir()));
    println!("Data:       {}", string_if_some(dirs::data_dir()));
    println!("Data Local: {}", string_if_some(dirs::data_local_dir()));
    println!("Executable: {}", string_if_some(dirs::executable_dir()));
}

fn string_if_some(input: Option<PathBuf>) -> String {
    match input {
        None => String::from("None"),
        Some(path) => path.display().to_string(),
    }
}
