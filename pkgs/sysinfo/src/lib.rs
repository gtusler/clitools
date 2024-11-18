use std::process;

use crate::sys_info::SysInfo;
use clap::{Arg, ArgMatches, Command};
use clap_complete::Shell;
use librs::cli::{
    cli_style::cli_style,
    gen_completion::{self, print_completions},
    output_format::OutputFormat,
};

mod sys_info;

pub fn command() -> Command {
    Command::new("sysinfo")
        .about("Get information about the system")
        .styles(cli_style())
        .arg(
            Arg::new("format")
                .short('f')
                .long("format")
                .help("How should the data be output? One of: print, json, json-pretty")
                .help_heading("Output Control")
                .default_value("print"),
        )
        .arg(gen_completion::arg())
}

pub fn handle(matches: &ArgMatches) -> i32 {
    if let Some(generator) = matches.get_one::<Shell>("generator").copied() {
        let mut cmd = command();
        print_completions(generator, &mut cmd);
        process::exit(0);
    }

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
