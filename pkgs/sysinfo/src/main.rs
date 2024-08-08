use crate::sys_info::SysInfo;
use clap::{Arg, Command};
use librs::cli::output_format::OutputFormat;

mod sys_info;

fn main() {
    let matches = Command::new("sysinfo")
        .about("Get information about the system")
        .arg(
            Arg::new("format")
                .short('f')
                .long("format")
                .help("How should the data be output? One of: print, json, json-pretty")
                .help_heading("Output Control")
                .default_value("print"),
        )
        .get_matches();

    let format_raw = matches.get_one::<String>("format");
    let output_format = OutputFormat::from_raw(format_raw);

    let info = SysInfo::get();

    match output_format {
        OutputFormat::Print => {
            println!("sysinfo\n");
            println!("{}", info);
        }
        OutputFormat::Json => {
            println!("{}", info.as_json());
        }
        OutputFormat::JsonPretty => {
            println!("{}", info.as_json_pretty());
        }
    }
}
