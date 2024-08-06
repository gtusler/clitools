use clap::Command;
use librs::sysinfo::{
    hardware::{
        battery::{get_battery_info, print_batteries_info},
        monitors::{get_monitors, print_monitors}
    },
    software::platform::UserPlatform
};

fn main() {
    let _ = Command::new("sysinfo")
        .about("Get information about the system")
        .get_matches();

    println!("sysinfo\n");

    let batteries_info = get_battery_info();
    print_batteries_info(batteries_info);

    println!("");

    let monitors = get_monitors();
    print_monitors(monitors);

    println!("");

    let platform = UserPlatform::detect();
    println!("Platform: {}", platform);
}
