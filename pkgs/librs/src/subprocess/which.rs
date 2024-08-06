use std::process::{Command, Stdio};

use crate::sysinfo::software::platform;

pub fn which(cmd_name: &str) -> Option<String> {
    let platform = platform::UserPlatform::detect();

    if platform.platform.contains("Linux") {
        return which_linux(cmd_name);
    }

    None
}

pub fn which_linux(cmd_name: &str) -> Option<String> {
    let which_child = Command::new("which")
        .arg(cmd_name)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start which process");

    let which_out = which_child.wait_with_output().expect("Failed to open which stdout");
    let which_ascii = which_out.stdout.as_ascii().expect("which output isn't ascii");
    let which_str = which_ascii.as_str();

    parse_which_linux(which_str)
}

/// Extract the path to the executable from the output of which
fn parse_which_linux(input: &str) -> Option<String> {
    if input.contains("not found") {
        return None;
    }

    Some(input.replace("\n", ""))
}

#[cfg(test)]
mod tests {
    use super::*;

    static LINUX_INPUT_FOUND: &str = "/usr/bin/zsh";
    static LINUX_INPUT_NOT_FOUND: &str = "something not found";

    #[test]
    fn it_parses_a_linux_path() {
        assert_eq!(parse_which_linux(LINUX_INPUT_FOUND), Some(String::from("/usr/bin/zsh")));
    }

    #[test]
    fn it_handles_linux_not_found() {
        assert_eq!(parse_which_linux(LINUX_INPUT_NOT_FOUND), None);
    }
}
