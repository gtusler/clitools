use std::fmt::Display;

use super::{bar_style::BarStyle, bar_width::BarWidth};

#[derive(Debug, Copy, Clone)]
pub struct ProgressBarTuiConfig {
    /// The max number of things to progress over
    pub max: usize,
    /// How big is a single step?
    pub step: usize,
    /// Whether or not to create the writable area
    pub writable_area: bool,
    /// Set the width of the progress bar
    pub bar_width: BarWidth,
    /// Describes the look of the progress bar
    pub bar_style: BarStyle,
    /// Whether or not to pause for input upon completion. If true, you don't need to manually call
    /// `.stop`
    pub pause_on_fin: bool,
}

impl ProgressBarTuiConfig {
    pub fn default() -> ProgressBarTuiConfig {
        ProgressBarTuiConfig {
            max: 100,
            step: 1,
            writable_area: true,
            bar_width: BarWidth::Full,
            bar_style: BarStyle::ArrowThick,
            pause_on_fin: true,
        }
    }
}

impl Display for ProgressBarTuiConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lines: Vec<String> = vec![
            format!("max: {}", self.max),
            format!("step: {}", self.step),
            format!("writable_area: {}", self.writable_area),
            format!("bar_width: {}", self.bar_width),
            format!("bar_style: {}", self.bar_style),
        ];

        write!(f, "{}", lines.join("\n"))
    }
}
