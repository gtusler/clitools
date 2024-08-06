use std::process::{Command, Stdio};

use crate::{sysinfo::software::platform, windows::windows_table::WindowsTable};

pub fn which(cmd_name: &str) -> Option<String> {
    let platform = platform::UserPlatform::detect();

    if platform.platform.contains("Linux") {
        return which_linux(cmd_name);
    }

    if platform.platform.contains("Windows") {
        return which_windows(cmd_name);
    }

    None
}

/// Extract the path to the executable from the system
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

/// Extract the path to the executable from the given string
fn parse_which_linux(input: &str) -> Option<String> {
    if input.contains("not found") {
        return None;
    }

    Some(input.replace("\n", ""))
}

pub fn which_windows(cmd_name: &str) -> Option<String> {
    let which_child = Command::new("Get-Command")
        .arg(cmd_name)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start Get-Command");

    let which_out = which_child.wait_with_output().expect("Failed to open Get-Command stdout");
    let which_ascii = which_out.stdout.as_ascii().expect("Get-Command output isn't ascii");
    let which_str = which_ascii.as_str();

    parse_which_linux(which_str)
}

pub fn parse_which_windows(input: &str) -> Option<String> {
    if input.contains("not recognized") {
        return None;
    }

    let table = WindowsTable::from_str(input);
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    static LINUX_INPUT_FOUND: &str = "/usr/bin/zsh";
    static LINUX_INPUT_NOT_FOUND: &str = "something not found";

    static WINDOWS_INPUT_FOUND: &str = "\
CommandType     Name                                               Version    Source
-----------     ----                                               -------    ------
Application     ipconfig.exe                                       10.0.2262… C:\\windows\\system32\\ipconfig.exe
";
    static WINDOWS_INPUT_NOT_FOUND: &str = "Get-Command: The term 'nothing' is not recognized as a name of a cmdlet, function, script file, or executable program.
Check the spelling of the name, or if a path was included, verify that the path is correct and try again.";

    #[test]
    fn it_parses_a_linux_path() {
        assert_eq!(parse_which_linux(LINUX_INPUT_FOUND), Some(String::from("/usr/bin/zsh")));
    }

    #[test]
    fn it_handles_linux_not_found() {
        assert_eq!(parse_which_linux(LINUX_INPUT_NOT_FOUND), None);
    }

    #[test]
    fn it_parses_a_windows_path() {
        assert_eq!(parse_which_windows(WINDOWS_INPUT_FOUND), Some(String::from("C:\\windows\\system32\\ipconfig.exe")));
    }

    #[test]
    fn it_handles_windows_not_found() {
        assert_eq!(parse_which_windows(WINDOWS_INPUT_NOT_FOUND), None);
    }
}
