use clap::{Arg, ArgMatches, Command};
use librs::cli::output_format::OutputFormat;
use crate::sys_info::SysInfo;

mod sys_info;

pub fn command() -> Command {
    Command::new("sysinfo")
        .about("Get information about the system")
        .arg(
            Arg::new("format")
                .short('f')
                .long("format")
                .help("How should the data be output? One of: print, json, json-pretty")
                .help_heading("Output Control")
                .default_value("print"),
        )
}

pub fn handle(matches: &ArgMatches) -> i32 {
    let format_raw = matches.get_one::<String>("format");
    let output_format = OutputFormat::from_raw(format_raw);

    let info = SysInfo::get();

    match output_format {
        OutputFormat::Print => {
            println!("sysinfo\n");
            println!("{}", info);
            0
        }
        OutputFormat::Json => {
            println!("{}", info.as_json());
            0
        }
        OutputFormat::JsonPretty => {
            println!("{}", info.as_json_pretty());
            0
        }
    }
}
