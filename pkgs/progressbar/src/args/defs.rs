use clap::{Arg, ArgAction};

pub fn max() -> Arg {
    Arg::new("max")
        .help("How many max are there in total?")
        .long("max")
        .default_value("100")
}

pub fn step() -> Arg {
    Arg::new("step")
        .help("How large is a single step?")
        .long("step")
        .default_value("1")
}

pub fn sleep() -> Arg {
    Arg::new("sleep")
        .help("How many seconds should we sleep between max?")
        .long("sleep")
        .default_value("1")
}

pub fn timed() -> Arg {
    Arg::new("timed")
        .help("Whether or not to generate eta")
        .action(ArgAction::SetTrue)
        .long("timed")
}
