use crate::{
    args::{self, parser::Args},
    progress_bars::progress::do_progress_progress,
};
use clap::{ArgMatches, Command};

pub fn command() -> Command {
    Command::new("progress")
        .about("use the `progress` package to draw progress bars")
        .arg_required_else_help(true)
        .arg(args::defs::max())
        .arg(args::defs::step())
        .arg(args::defs::sleep())
        .arg(args::defs::timed())
}

pub fn handle(matches: &ArgMatches) -> i32 {
    let args = Args::from_matches(matches);
    do_progress_progress(args.max, args.step, args.sleep);

    0
}
