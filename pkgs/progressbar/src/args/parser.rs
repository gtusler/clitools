use std::{fmt::Display, process};

use clap::ArgMatches;

pub struct Args {
    pub max: usize,
    pub step: usize,
    pub sleep: usize,
    pub timed: bool,
}

impl Args {
    pub fn from_matches(matches: &ArgMatches) -> Args {
        let max_str = matches.get_one::<String>("max").expect("no arg `max`");
        let step_str = matches.get_one::<String>("step").expect("no arg `step`");
        let sleep_str = matches.get_one::<String>("sleep").expect("no arg `sleep`");
        let timed = matches.get_flag("timed");
        let max = max_str.parse::<usize>();
        let step = step_str.parse::<usize>();
        let sleep = sleep_str.parse::<usize>();

        if max.is_err() {
            println!("argument `max` must be a whole, positive number");
            process::exit(1);
        };

        if step.is_err() {
            println!("argument `step` must be a whole, positive number");
            process::exit(1);
        }

        if sleep.is_err() {
            println!("argument `sleep` must be a whole, positive number");
            process::exit(1);
        }

        let max = max.unwrap();
        let step = step.unwrap();
        let sleep = sleep.unwrap();

        Args {
            max,
            step,
            sleep,
            timed,
        }
    }
}

impl Display for Args {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lines: Vec<String> = vec![
            format!("max: {}", self.max),
            format!("step: {}", self.step),
            format!("sleep: {}", self.sleep),
        ];

        write!(f, "{}", lines.join("\n"))
    }
}
