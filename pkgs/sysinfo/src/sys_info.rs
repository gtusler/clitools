use std::fmt::Display;
use librs::{subprocess::uname::Uname, sysinfo::{
    hardware::{
        battery::{get_battery_info, BatteryInfo},
        monitors::{get_monitors, GetMonitorError, Monitor},
    },
    software::platform::UserPlatform,
}};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SysInfo {
    platform: UserPlatform,
    batteries: Vec<BatteryInfo>,
    monitors: Result<Vec<Monitor>, GetMonitorError>,
    uname: Option<Uname>,
}

impl SysInfo {
    pub fn get() -> SysInfo {
        let platform = UserPlatform::detect();
        let batteries = get_battery_info();
        let monitors = get_monitors();
        let uname = if platform.platform.contains("Linux") {
            Some(Uname::spawn_and_parse())
        } else {
            None
        };

        SysInfo {
            platform,
            batteries,
            monitors,
            uname,
        }
    }

    pub fn as_json(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize SysInfo as json")
    }

    pub fn as_json_pretty(&self) -> String {
        serde_json::to_string_pretty(self).expect("Failed to serialize SysInfo as pretty json")
    }
}

impl Display for SysInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let platform = format!("Platform: {}", self.platform);

        let mut lines: Vec<String> = Vec::new();

        if let Some(uname) = &self.uname {
            lines.push(format!("uname:\n{}", uname));
            lines.push(String::from(""));
        }
        lines.push(platform);

        lines.push(String::from(""));
        if self.batteries.len() == 1 {
            lines.push(String::from("1 battery"));
        } else {
            lines.push(format!("{} batteries", self.batteries.len()));
        }
        for battery in self.batteries.iter() {
            lines.push(format!("{}", battery));
        }

        lines.push(String::from(""));
        match &self.monitors {
            Ok(monitors) => {
                if monitors.len() == 1 {
                    lines.push(String::from("1 monitor"));
                } else {
                    lines.push(format!("{} monitors", monitors.len()));
                }
                for monitor in monitors {
                    lines.push(format!("{}", monitor));
                }
            }
            Err(err) => {
                lines.push(format!("{}", err));
            }
        }

        let lines = lines.join("\n");

        write!(f, "{}", lines)
    }
}
