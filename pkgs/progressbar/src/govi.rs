use std::{process, sync::mpsc::channel, thread, time::Duration};

use librs::progressbar::{
    bar_style::BarStyle, bar_width::BarWidth, tui::ProgressBarTui, tui_config::ProgressBarTuiConfig,
};

use crate::progressing::Style;

pub fn do_progress_govi(style: Style, max: usize, step: usize, sleep: usize) -> () {
    let bar_style: BarStyle = style.into();
    let mut progress_controller = ProgressBarTui::with(ProgressBarTuiConfig {
        max,
        step,
        writable_area: true,
        bar_width: BarWidth::Full,
        bar_style,
        pause_on_fin: true,
    });
    let mut exited = false;
    let started = progress_controller.start();

    match started {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Failed to start: {}", e);
            process::exit(1);
        }
    }

    let (tx, _rx) = channel();
    ctrlc::set_handler(move || {
        exited = true;
        tx.send(()).expect("Could not send signal on channel")
    })
    .expect("Error setting Ctrl-c handler");

    // rx.recv().expect("Could not receive from channel");

    let sleep_duration = Duration::from_secs(sleep.try_into().unwrap());

    // for i in 1..max {
    for i in 1..max + 1 {
        if exited {
            let _ = progress_controller.stop();
            eprintln!("interrupt");
            process::exit(1);
        }
        // println!("{}", i);
        let _ = progress_controller.tick(Some(format!("step {}", i.to_string())));
        thread::sleep(sleep_duration);
    }

    // let stopped = progress_controller.stop();
    //
    // match stopped {
    //     Ok(_) => (),
    //     Err(e) => {
    //         println!("Failed to stop: {}", e);
    //         process::exit(1);
    //     }
    // }
}
