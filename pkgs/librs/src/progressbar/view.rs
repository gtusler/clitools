use super::bar_style::BarStyle;
use progressing::{mapping::Bar as MappingBar, mapping::Config as MappingConfig, Baring};

pub struct ProgressBarView {
    pub style: BarStyle,
    pub width: u16,
    pub max_steps: usize,
}

impl ProgressBarView {
    pub fn draw(&self, current: usize) -> String {
        let width: usize = self.width.into();

        let mut progress_bar = MappingBar::with(MappingConfig {
            bar_len: width - 8,
            style: self.style.to_pattern(),
            interesting_progress_step: 0.1,
            min_k: 1,
            max_k: self.max_steps,
        });

        progress_bar.set(current);

        format!("{}", progress_bar)
    }
}
