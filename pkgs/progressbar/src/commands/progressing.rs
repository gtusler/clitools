use crate::{
    args::{self, parser::Args},
    progress_bars::progressing::{do_progress_progressing, BarType, Style},
};
use clap::{Arg, ArgMatches, Command};

pub fn command() -> Command {
    Command::new("progressing")
        .about("use the `progressing` package to draw progress bars")
        .arg_required_else_help(true)
        .arg(args::defs::max())
        .arg(args::defs::step())
        .arg(args::defs::sleep())
        .arg(args::defs::timed())
        .arg(
            Arg::new("type")
                .help("What type of progress bar should be used? clamped, mapped, bernoulli")
                .long("type")
        )
        .arg(
            Arg::new("style")
                .help("Sets the visual style of the bar. hashes-plain, hashes-dashes, arrow-thin, arrow-thick")
                .long("style")
                .default_value("arrow-thin")
        )
}

pub fn handle(matches: &ArgMatches) -> i32 {
    let args = Args::from_matches(matches);

    let bar_type_str = match matches.get_one::<String>("type") {
        Some(s) => s,
        None => "clamped",
    };
    let bar_type = BarType::from(bar_type_str.to_string());

    let style_str = matches.get_one::<String>("style").unwrap();
    let style: Style = style_str.into();

    do_progress_progressing(bar_type, style, args.max, args.step, args.sleep, args.timed);

    0
}
