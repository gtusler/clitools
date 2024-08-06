#![feature(ascii_char)]

pub mod math;
pub mod rot;
pub mod sysinfo;
pub mod subprocess;

pub fn from_librs() -> String {
    String::from("This string comes from librs")
}
