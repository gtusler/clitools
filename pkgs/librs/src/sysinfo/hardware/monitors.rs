use crate::subprocess::which::which;
use serde::Serialize;
use std::{
    error::Error,
    fmt::Display,
    process::{Command, Stdio},
};

#[derive(Debug, Serialize)]
pub enum GetMonitorError {
    NoUsableCommand,
}

impl Error for GetMonitorError {}

impl Display for GetMonitorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            GetMonitorError::NoUsableCommand => {
                "Failed to find a program to detect monitor information"
            }
        };
        write!(f, "{}", msg)
    }
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Monitor {
    pub name: String,
    pub is_primary: bool,
    pub pixel_width: u16,
    pub pixel_height: u16,
    pub real_width: u16,
    pub real_height: u16,
}

impl Display for Monitor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let suffix = if self.is_primary { " primary" } else { "" };
        write!(
            f,
            "{} ({}x{}) ({}mm x {}mm){}",
            self.name,
            self.pixel_width,
            self.pixel_height,
            self.real_width,
            self.real_height,
            suffix
        )
    }
}

pub fn print_monitors(monitors: Vec<Monitor>) {
    let suffix = if monitors.len() == 1 { "" } else { "s" };
    println!("{} monitor{}", monitors.len(), suffix);

    for monitor in monitors {
        println!("{}", monitor);
    }
}

pub fn get_monitors() -> Result<Vec<Monitor>, GetMonitorError> {
    let which_xrandr = which("xrandr");
    match which_xrandr {
        Some(_) => Ok(get_monitors_xrandr()),
        None => Err(GetMonitorError::NoUsableCommand),
    }
}

pub fn get_monitors_xrandr() -> Vec<Monitor> {
    let xrandr_child = Command::new("xrandr")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start xrandr process");

    let xrandr_out = xrandr_child
        .wait_with_output()
        .expect("Failed to open xrandr stdout");
    let xrandr_ascii = xrandr_out.stdout.as_ascii().expect("That aint ascii");
    let xrandr_str = xrandr_ascii.as_str();
    parse_xrandr(xrandr_str)
}

pub fn parse_xrandr(input: &str) -> Vec<Monitor> {
    let split = input.split('\n');

    let meaningful: Vec<&str> = split
        .filter(|line| !line.starts_with("Screen"))
        .filter(|line| !line.starts_with("  "))
        .filter(|line| !line.is_empty())
        .collect();

    let mut output: Vec<Monitor> = vec![];

    for line in meaningful {
        let split: Vec<&str> = line.split(" ").filter(|word| *word != "primary").collect();

        let name = split.first().expect("Unable to parse name").to_string();
        let is_primary = line.contains("primary");
        let real_width_str = split
            .get(11)
            .expect("Unable to find real width")
            .to_string()
            .replace("mm", "");
        let real_width: u16 = real_width_str.parse().expect("Failed to parse real width");
        let real_height_str = split
            .get(13)
            .expect("Unable to find real height")
            .to_string()
            .replace("mm", "");
        let real_height: u16 = real_height_str
            .parse()
            .expect("Failed to parse real height");
        let dimensions_line = split
            .get(2)
            .expect("Unable to find dimensions line")
            .to_string();
        let pixel_dimensions = parse_xrandr_dimensions(dimensions_line);

        output.push(Monitor {
            name,
            is_primary,
            pixel_width: pixel_dimensions.0,
            pixel_height: pixel_dimensions.1,
            real_width,
            real_height,
        });
    }

    output
}

pub fn parse_xrandr_dimensions(input: String) -> (u16, u16) {
    let split_top: Vec<&str> = input.split("+").collect();
    let dimensions_str = split_top
        .first()
        .expect("Failed to find dimensions in line");
    let dimensions_split: Vec<&str> = dimensions_str.split("x").collect();

    let width_str = dimensions_split
        .first()
        .expect("Failed to find width in line");
    let height_str = dimensions_split
        .get(1)
        .expect("Failed to find height in line");

    let width: u16 = width_str.parse().expect("Failed to parse pixel width");
    let height: u16 = height_str.parse().expect("Failed to parse pixel height");

    (width, height)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "Screen 0: minimum 16 x 16, current 5760 x 2160, maximum 32767 x 32767
XWAYLAND0 connected primary 1920x1080+0+1080 (normal left inverted right x axis y axis) 340mm x 190mm
   1920x1080     59.96*+
   1440x1080     59.87  
   1400x1050     59.86  
   1280x1024     59.89  
   1280x960      59.94  
   1152x864      59.78  
   1024x768      59.92  
   800x600       59.86  
   640x480       59.38  
   320x240       59.52  
   1680x1050     59.95  
   1440x900      59.89  
   1280x800      59.81  
   720x480       59.71  
   640x400       59.20  
   320x200       58.96  
   1600x900      59.95  
   1368x768      59.88  
   1280x720      59.86  
   1024x576      59.90  
   864x486       59.92  
   720x400       59.55  
   640x350       59.77  
XWAYLAND1 connected 3840x2160+1920+0 (normal left inverted right x axis y axis) 600mm x 340mm
   3840x2160     59.98*+
   2048x1536     59.95  
   1920x1440     59.97  
   1600x1200     59.87  
   1440x1080     59.99  
   1400x1050     59.98  
   1280x1024     59.89  
   1280x960      59.94  
   1152x864      59.96  
   1024x768      59.92  
   800x600       59.86  
   640x480       59.38  
   320x240       59.52  
   2560x1600     59.99  
   1920x1200     59.88  
   1680x1050     59.95  
   1440x900      59.89  
   1280x800      59.81  
   720x480       59.71  
   640x400       59.95  
   320x200       58.96  
   3200x1800     59.96  
   2880x1620     59.96  
   2560x1440     59.96  
   2048x1152     59.90  
   1920x1080     59.96  
   1600x900      59.95  
   1368x768      59.88  
   1280x720      59.86  
   1024x576      59.90  
   864x486       59.92  
   720x400       59.55  
   640x350       59.77";

    #[test]
    fn it_does_the_thing() {
        let output = vec![
            Monitor {
                name: String::from("XWAYLAND0"),
                is_primary: true,
                pixel_width: 1920,
                pixel_height: 1080,
                real_width: 340,
                real_height: 190,
            },
            Monitor {
                name: String::from("XWAYLAND1"),
                is_primary: false,
                pixel_width: 3840,
                pixel_height: 2160,
                real_width: 600,
                real_height: 340,
            },
        ];
        assert_eq!(parse_xrandr(INPUT), output);
    }
}
