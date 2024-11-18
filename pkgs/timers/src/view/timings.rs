use crate::db::entity::timing::Timing;
use chrono::{Datelike, Timelike};
use color_print::{cformat, cprintln};
use librs::datetime::datetime_comparison::is_same_ymd;
use pretty_duration::{pretty_duration, PrettyDurationOptions, PrettyDurationOutputFormat};
use std::time::Duration;

pub fn print_timings(timings: Vec<Timing>) {
    let mut durations: Vec<Duration> = Vec::new();
    let pretty_duration_options = Some(PrettyDurationOptions {
        output_format: Some(PrettyDurationOutputFormat::Compact),
        singular_labels: None,
        plural_labels: None,
    });

    for timing in &timings {
        let duration = match (timing.stop - timing.start).to_std() {
            Ok(dur) => dur,
            Err(e) => {
                eprintln!("{}", e);
                return;
            }
        };
        let duration_pretty = pretty_duration(&duration, pretty_duration_options.clone());

        /*
        What do I want to do here?

        If I have a start and stop on the same date, print like:
        2024-10-10  20:30:23 -> 22:30:12  1hr 59min 49sec

        That cleans things up nicely.

        As for timings spanning multiple days, that's a bridge I'll cross when I come to it.
        These situations won't arise often in reality.
         */

        if is_same_ymd(timing.start, timing.stop) {
            println!("{}", print_same_day(timing, &duration_pretty));
        } else {
            println!("start {}", timing.start);
            println!("stop  {}", timing.stop);
            cprintln!("<y>{}</>", duration_pretty);
        }

        durations.push(duration);
    }

    println!();
    cprintln!("Sessions: <y>{}</>", timings.len());

    let duration_total: Duration = durations.into_iter().sum();

    // println!("{:#?}", durations);
    cprintln!(
        "Total: <y>{}</>",
        pretty_duration(&duration_total, pretty_duration_options)
    );
}

fn print_same_day(timing: &Timing, duration_pretty: &str) -> String {
    let naive_start = timing.start.naive_utc();
    let naive_stop = timing.stop.naive_utc();

    cformat!(
        "{}-{}-{}  {:02}:{:02}:{:02} -> {:02}:{:02}:{:02}  <y>{}</y>",
        naive_start.year(),
        naive_start.month(),
        naive_start.day(),
        naive_start.hour(),
        naive_start.minute(),
        naive_start.second(),
        naive_stop.hour(),
        naive_stop.minute(),
        naive_stop.second(),
        duration_pretty,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    mod print_same_day {
        use super::*;
        use chrono::{TimeZone, Utc};

        #[test]
        fn easy() {
            assert_eq!(
                print_same_day(
                    &Timing {
                        start: Utc.with_ymd_and_hms(2024, 10, 10, 20, 30, 32).unwrap(),
                        stop: Utc.with_ymd_and_hms(2024, 10, 10, 22, 30, 18).unwrap(),
                        project: String::from("something"),
                    },
                    "10s"
                ),
                cformat!("2024-10-10  20:30:32 -> 22:30:18  <y>10s</>")
            );
        }

        #[test]
        fn it_pads_numbers() {
            assert_eq!(
                print_same_day(
                    &Timing {
                        start: Utc.with_ymd_and_hms(2024, 10, 8, 7, 0, 3).unwrap(),
                        stop: Utc.with_ymd_and_hms(2024, 10, 8, 9, 8, 7).unwrap(),
                        project: String::from("something"),
                    },
                    "10s"
                ),
                cformat!("2024-10-8  07:00:03 -> 09:08:07  <y>10s</>")
            );
        }
    }
}
