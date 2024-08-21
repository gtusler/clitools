use std::{process, str::FromStr, thread, time::Duration};

use librs::progressbar::bar_style::BarStyle;
use progressing::{
    clamping::Bar as ClampingBar, clamping::Config as ClampingConfig, mapping::Bar as MappingBar,
    mapping::Config as MappingConfig, Baring,
};

pub enum BarType {
    Clamped,
    Bernoulli,
    Mapped,
}

impl From<String> for BarType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "clamped" => BarType::Clamped,
            "bernoulli" => BarType::Bernoulli,
            "mapped" => BarType::Mapped,
            _ => {
                println!("invalid progressing BarType: {}", value);
                process::exit(1);
            }
        }
    }
}

impl FromStr for BarType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match &*s.to_ascii_lowercase() {
            "clamped" => BarType::Clamped,
            "bernoulli" => BarType::Bernoulli,
            "mapped" => BarType::Mapped,
            _ => return Err(format!("unknown bar type: `{s}`")),
        })
    }
}

pub enum Style {
    HashesPlain,
    HashesDashes,
    ArrowThin,
    ArrowThick,
}

impl Style {
    pub fn to_pattern(&self) -> String {
        match self {
            Style::HashesPlain => String::from("[#  ]"),
            Style::HashesDashes => String::from("(#--)"),
            Style::ArrowThin => String::from("(->.)"),
            Style::ArrowThick => String::from("(=>.)"),
        }
    }
}

impl FromStr for Style {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match &*s.to_ascii_lowercase() {
            "hashes-plain" => Style::HashesPlain,
            "hashes-dashes" => Style::HashesDashes,
            "arrow-thin" => Style::ArrowThin,
            "arrow-thick" => Style::ArrowThick,
            _ => return Err(format!("unknown style: `{s}`")),
        })
    }
}

impl From<String> for Style {
    fn from(value: String) -> Self {
        match value.as_str() {
            "hashes-plain" => Style::HashesPlain,
            "hashes-dashes" => Style::HashesDashes,
            "arrow-thin" => Style::ArrowThin,
            "arrow-thick" => Style::ArrowThick,
            _ => {
                println!("invalid progressing style: {}", value);
                process::exit(1);
            }
        }
    }
}

impl From<&String> for Style {
    fn from(value: &String) -> Self {
        match value.as_str() {
            "hashes-plain" => Style::HashesPlain,
            "hashes-dashes" => Style::HashesDashes,
            "arrow-thin" => Style::ArrowThin,
            "arrow-thick" => Style::ArrowThick,
            _ => {
                println!("invalid progressing style: {}", value);
                process::exit(1);
            }
        }
    }
}

impl Into<BarStyle> for Style {
    fn into(self) -> BarStyle {
        match self {
            Style::HashesPlain => BarStyle::HashesPlain,
            Style::HashesDashes => BarStyle::HashesDashes,
            Style::ArrowThin => BarStyle::ArrowThin,
            Style::ArrowThick => BarStyle::ArrowThick,
        }
    }
}

pub fn do_progress_progressing(
    bar_type: BarType,
    style: Style,
    max: usize,
    step: usize,
    sleep: usize,
    timed: bool,
) -> () {
    match bar_type {
        BarType::Clamped => do_clamped(style, max, step, sleep, timed),
        BarType::Mapped => do_mapped(style, max, step, sleep, timed),
        BarType::Bernoulli => do_bernoulli(style, max, step, sleep, timed),
    }
}

fn do_clamped(style: Style, max: usize, step: usize, sleep: usize, timed: bool) -> () {
    let sleep_u64 = u64::try_from(sleep).unwrap();
    let sleep_duration = Duration::from_secs(sleep_u64);

    if timed {
        do_clamped_timed(style, max, step, sleep_duration);
    } else {
        do_clamped_untimed(style, max, step, sleep_duration);
    }
}

fn do_clamped_timed(style: Style, max: usize, step: usize, sleep: Duration) -> () {
    let mut progress_bar = ClampingBar::with(ClampingConfig {
        bar_len: max,
        style: style.to_pattern(),
        interesting_progress_step: 0.1,
    })
    .timed();

    let mut current_count: usize = 0;

    while current_count < max {
        thread::sleep(sleep);

        current_count += step;
        let thing: f64 = current_count as f64 / max as f64;
        progress_bar.set(thing);
        println!("{}", progress_bar);
    }
}

fn do_clamped_untimed(style: Style, max: usize, step: usize, sleep: Duration) -> () {
    let mut progress_bar = ClampingBar::with(ClampingConfig {
        bar_len: max,
        style: style.to_pattern(),
        interesting_progress_step: 0.1,
    });

    let mut current_count: usize = 0;

    while current_count < max {
        thread::sleep(sleep);

        current_count += step;
        let thing: f64 = current_count as f64 / max as f64;
        progress_bar.set(thing);
        println!("{}", progress_bar);
    }
}

fn do_mapped(style: Style, max: usize, step: usize, sleep: usize, timed: bool) -> () {
    let sleep_u64 = u64::try_from(sleep).unwrap();
    let sleep_duration = Duration::from_secs(sleep_u64);

    if timed {
        do_mapped_timed(style, max, step, sleep_duration);
    } else {
        do_mapped_untimed(style, max, step, sleep_duration);
    }
}

fn do_mapped_timed(style: Style, max: usize, step: usize, sleep: Duration) -> () {
    let mut progress_bar = MappingBar::with(MappingConfig {
        bar_len: max,
        style: style.to_pattern(),
        interesting_progress_step: 0.1,
        min_k: 1,
        max_k: max,
    });

    let mut current_count: usize = 0;

    while current_count < max {
        thread::sleep(sleep);

        current_count += step;
        progress_bar.set(current_count);
        println!("{}", progress_bar);
    }
}

fn do_mapped_untimed(style: Style, max: usize, step: usize, sleep: Duration) -> () {
    let mut progress_bar = MappingBar::with(MappingConfig {
        bar_len: max,
        style: style.to_pattern(),
        interesting_progress_step: 0.1,
        min_k: 1,
        max_k: max,
    });

    let mut current_count: usize = 0;

    while current_count < max {
        thread::sleep(sleep);

        current_count += step;
        progress_bar.set(current_count);
        println!("{}", progress_bar);
    }
}

fn do_bernoulli(style: Style, max: usize, step: usize, sleep: usize, timed: bool) -> () {
    let sleep_u64 = u64::try_from(sleep).unwrap();
    let sleep_duration = Duration::from_secs(sleep_u64);

    if timed {
        do_bernoulli_timed(style, max, step, sleep_duration);
    } else {
        do_bernoulli_untimed(style, max, step, sleep_duration);
    }
}

fn do_bernoulli_timed(_style: Style, _max: usize, _step: usize, _sleep: Duration) -> () {
    todo!()
}

fn do_bernoulli_untimed(_style: Style, _max: usize, _step: usize, _sleep: Duration) -> () {
    todo!()
}
