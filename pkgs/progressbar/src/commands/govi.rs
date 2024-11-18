use crate::{
    args::{self, parser::Args},
    progress_bars::{govi::do_progress_govi, progressing::Style},
};
use clap::{Arg, ArgMatches, Command};

pub fn command() -> Command {
    Command::new("govi")
        .about("use a handrolled progress bar solution")
        // .arg_required_else_help(true)
        .arg(args::defs::max())
        .arg(args::defs::step())
        .arg(args::defs::sleep())
        .arg(args::defs::timed())
        .arg(
            Arg::new("style")
                .help("Sets the visual style of the bar. hashes-plain, hashes-dashes, arrow-thin, arrow-thick")
                .long("style")
                .default_value("arrow-thin")
        )
}

pub fn handle(matches: &ArgMatches) -> i32 {
    let args = Args::from_matches(matches);

    let style_str = matches.get_one::<String>("style").unwrap();
    let style: Style = style_str.into();

    do_progress_govi(style, args.max, args.step, args.sleep);

    0
}
