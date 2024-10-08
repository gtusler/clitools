use serde::Serialize;
use std::fmt::Display;

#[derive(Debug, Serialize)]
pub struct UserPlatform {
    pub platform: String,
    pub is_x86: bool,
    pub is_x64: bool,
}

impl UserPlatform {
    pub fn detect() -> UserPlatform {
        let user_plat = platform_lp::Platform::get_user_platform();
        let is_x86 = platform_lp::Architecture::X32 == user_plat;
        let is_x64 = platform_lp::Architecture::X64 == user_plat;

        UserPlatform {
            platform: user_plat.to_string(),
            is_x86,
            is_x64,
        }
    }

    pub fn is_windows(&self) -> bool {
        !self.platform.contains("Linux")
    }

    pub fn is_linux(&self) -> bool {
        self.platform.contains("Linux")
    }
}

impl Display for UserPlatform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.platform)
    }
}
