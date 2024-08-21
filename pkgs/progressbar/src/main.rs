use args::Args;
use clap::{Arg, ArgAction, ArgMatches, Command};
use govi::do_progress_govi;
use ml_progress::do_progress_ml_progress;
use progress::do_progress_progress;
use progressing::{do_progress_progressing, BarType, Style};

mod args;
mod govi;
mod ml_progress;
mod progress;
mod progressing;

fn main() {
    let max_arg = Arg::new("max")
        .help("How many max are there in total?")
        .long("max")
        .default_value("100");

    let step_arg = Arg::new("step")
        .help("How large is a single step?")
        .long("step")
        .default_value("1");

    let sleep_arg = Arg::new("sleep")
        .help("How many seconds should we sleep between max?")
        .long("sleep")
        .default_value("1");

    let timed_arg = Arg::new("timed")
        .help("Whether or not to generate eta")
        .action(ArgAction::SetTrue)
        .long("timed");

    let matches = Command::new("progress")
        .about("An exploration of termnial progress bars")
        .version("0.1.0")
        .subcommand_required(true)
        .subcommand(
            Command::new("progress")
                .about("use the `progress` package to draw progress bars")
                .arg_required_else_help(true)
                .arg(&max_arg)
                .arg(&step_arg)
                .arg(&sleep_arg)
                .arg(&timed_arg)
        )
        .subcommand(
            Command::new("ml-progress")
                .about("use the `ml-progress` package to draw progress bars")
                .arg_required_else_help(true)
                .arg(&max_arg)
                .arg(&step_arg)
                .arg(&sleep_arg)
                .arg(&timed_arg)
        )
        .subcommand(
            Command::new("progressing")
                .about("use the `progressing` package to draw progress bars")
                .arg_required_else_help(true)
                .arg(&max_arg)
                .arg(&step_arg)
                .arg(&sleep_arg)
                .arg(&timed_arg)
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
        )
        .subcommand(
            Command::new("govi")
                .about("use a handrolled progress bar solution")
                // .arg_required_else_help(true)
                .arg(&max_arg)
                .arg(&step_arg)
                .arg(&sleep_arg)
                .arg(&timed_arg)
                .arg(
                    Arg::new("style")
                        .help("Sets the visual style of the bar. hashes-plain, hashes-dashes, arrow-thin, arrow-thick")
                        .long("style")
                        .default_value("arrow-thin")
                )
        )
        .get_matches();

    match matches.subcommand() {
        Some(("progress", progress_matches)) => handle_progress_progress(progress_matches),
        Some(("ml-progress", ml_progress_matches)) => {
            handle_progress_ml_progress(ml_progress_matches)
        }
        Some(("progressing", progressing_matches)) => {
            handle_progress_progressing(progressing_matches)
        }
        Some(("govi", govi_matches)) => handle_progress_govi(govi_matches),
        _ => unreachable!(),
    }
}

fn handle_progress_progress(matches: &ArgMatches) -> () {
    let args = Args::from_matches(matches);
    do_progress_progress(args.max, args.step, args.sleep);
}

fn handle_progress_ml_progress(matches: &ArgMatches) -> () {
    let args = Args::from_matches(matches);
    do_progress_ml_progress(args.max, args.step, args.sleep);
}

fn handle_progress_progressing(matches: &ArgMatches) -> () {
    let args = Args::from_matches(matches);

    let bar_type_str = match matches.get_one::<String>("type") {
        Some(s) => s,
        None => "clamped",
    };
    let bar_type = BarType::from(bar_type_str.to_string());

    let style_str = matches.get_one::<String>("style").unwrap();
    let style: Style = style_str.into();

    do_progress_progressing(bar_type, style, args.max, args.step, args.sleep, args.timed);
}

fn handle_progress_govi(matches: &ArgMatches) -> () {
    let args = Args::from_matches(matches);

    let style_str = matches.get_one::<String>("style").unwrap();
    let style: Style = style_str.into();

    do_progress_govi(style, args.max, args.step, args.sleep);
}
