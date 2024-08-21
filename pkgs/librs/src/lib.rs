#![feature(ascii_char)]

pub mod cli;
pub mod distro;
pub mod math;
pub mod progressbar;
pub mod rot;
pub mod subprocess;
pub mod sysinfo;
pub mod windows;

pub fn from_librs() -> String {
    String::from("This string comes from librs")
}
